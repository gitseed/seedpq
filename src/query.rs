use std::option::Option;
use std::sync::mpsc::Receiver;

use hybrid_array::typenum::U0;
use hybrid_array::typenum::Unsigned;
use hybrid_array::{Array, ArraySize};

use crate::connection_raw::SendableQueryResult;
use crate::query_error::{QueryDataError, QueryError};
use crate::query_raw::{ExecStatusType, PQresStatus, RawQueryResult};

pub trait QueryResult<'a>:
    TryFrom<Array<Option<&'a [u8]>, Self::Columns>, Error = QueryDataError>
{
    type Columns: ArraySize;
    const COLUMN_NAMES: Array<&'static str, Self::Columns>;
}

impl<T> Iterator for QueryReceiver<T>
where
    for<'a> T: QueryResult<'a>,
{
    type Item = Result<T, QueryError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.columns {
            None => match self.recv.recv() {
                Err(e) => Some(Err(e.into())),
                Ok(r) => {
                    let r: &mut RawQueryResult = self.current_raw_query_result.insert(r.into());
                    let status = r.PQresultStatus();
                    match status {
                        ExecStatusType::PGRES_COMMAND_OK
                        | ExecStatusType::PGRES_TUPLES_OK
                        | ExecStatusType::PGRES_SINGLE_TUPLE
                        | ExecStatusType::PGRES_TUPLES_CHUNK => {
                            let columns: &mut usize = self.columns.insert(r.PQnfields());
                            if *columns < <T::Columns as Unsigned>::to_usize() {
                                Some(Err(QueryError::InsufficientColumnsError {
                                    query: self.query.clone(),
                                    expected: <T::Columns as Unsigned>::to_usize(),
                                    found: *columns,
                                }))
                            } else {
                                for column_number in 0..T::COLUMN_NAMES.len() {
                                    let expected_column_name: &'static str = T::COLUMN_NAMES[column_number];
                                    let actual_column_name = r.PQfname(column_number);
                                    if expected_column_name != actual_column_name {
                                        return Some(Err(QueryError::ColumnNameMismatchError {
                                            query: self.query.clone(),
                                            column_number,
                                            expected: expected_column_name,
                                            found: actual_column_name,
                                        }));
                                    }
                                };

                                let rows: usize = r.PQntuples();
                                if rows == 0 {
                                    None
                                } else if rows == self.current_row {
                                    // todo: Will be more complicated once we start using chunked rows mode.
                                    None
                                } else {
                                    let data: Array<Option<&[u8]>, T::Columns> =
                                        Array::from_fn(|column| {
                                            r.fetch_cell(self.current_row, column)
                                        });
                                    self.current_row += 1;
                                    match T::try_from(data) {
                                        Ok(result) => Some(Ok(result)),
                                        Err(e) => Some(Err(QueryError::QueryDataError {
                                            e,
                                            query: self.query.clone(),
                                        })),
                                    }
                                }
                            }
                        }
                        _ => Some(Err(QueryError::ResultStatusError {
                            status: PQresStatus(status),
                            query: self.query.clone(),
                        })),
                    }
                }
            },
            Some(_) => {
                match &mut self.current_raw_query_result {
                    None => todo!(), // Required for chunked rows mode
                    Some(r) => {
                        let rows: usize = r.PQntuples();
                        if rows == 0 {
                            None
                        } else if rows == self.current_row {
                            // todo: Will be more complicated once we start using chunked rows mode.
                            None
                        } else {
                            let data: Array<Option<&[u8]>, T::Columns> =
                                Array::from_fn(|column| r.fetch_cell(self.current_row, column));
                            self.current_row += 1;
                            match T::try_from(data) {
                                Ok(result) => Some(Ok(result)),
                                Err(e) => Some(Err(QueryError::QueryDataError {
                                    e,
                                    query: self.query.clone(),
                                })),
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct EmptyResult;

impl QueryResult<'_> for EmptyResult {
    type Columns = U0;
    const COLUMN_NAMES: Array<&'static str, Self::Columns> = Array([]);
}

impl TryFrom<Array<Option<&[u8]>, U0>> for EmptyResult {
    type Error = QueryDataError;

    fn try_from(_: Array<Option<&[u8]>, U0>) -> Result<Self, Self::Error> {
        unreachable!()
    }
}

/// Receives results from a single query, from the database connection thread.
/// Implements Iterator for Result<T, Error>.
/// The methods of this struct may block. Including next().
#[derive(Debug)]
pub struct QueryReceiver<T> {
    pub(crate) query: String,
    pub(crate) recv: Receiver<SendableQueryResult>,
    pub(crate) phantom: std::marker::PhantomData<T>,
    pub(crate) current_raw_query_result: Option<RawQueryResult>,
    pub(crate) current_row: usize,
    // If this is none that means nothing has been fetched yet.
    // This is the number of columns actually returned by the query.
    // This may be a different value than what T is expecting.
    pub(crate) columns: Option<usize>,
}

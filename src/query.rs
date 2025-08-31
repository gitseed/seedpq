use std::option::Option;
use std::sync::mpsc::Receiver;

use hybrid_array::typenum::Unsigned;
use hybrid_array::{Array, ArraySize};

use crate::connection_raw::SendableQueryResult;
use crate::query_error::{QueryDataError, QueryError};
use crate::query_raw::RawQueryResult;

pub trait QueryResult<'a>:
    TryFrom<Array<Option<&'a [u8]>, Self::Columns>, Error = QueryDataError>
{
    type Columns: ArraySize;
}

impl<T> Iterator for QueryReceiver<T>
where
    for<'a> T: QueryResult<'a>,
{
    type Item = Result<T, QueryError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_row += 1;
        match self.columns {
            None => match self.recv.recv() {
                Err(e) => Some(Err(QueryError::RecvError(e))),
                Ok(r) => {
                    let r: &mut RawQueryResult = self.current_raw_query_result.insert(r.into());
                    let columns: &mut usize = self.columns.insert(r.PQnfields());
                    if *columns < <T::Columns as Unsigned>::to_usize() {
                        Some(Err(QueryError::InsufficientColumnsError {
                            query: self.query.clone(),
                            expected: <T::Columns as Unsigned>::to_usize(),
                            found: *columns,
                        }))
                    } else {
                        let rows: usize = r.PQntuples();
                        if rows == 0 {
                            None
                        } else if rows == self.current_row {
                            // todo: Will be more complicated once we start using chunked rows mode.
                            None
                        } else {
                            let data: Array<Option<&[u8]>, T::Columns> =
                                Array::from_fn(|column| r.fetch_cell(self.current_row, column));
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

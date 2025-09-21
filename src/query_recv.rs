use std::option::Option;
use std::sync::mpsc::Receiver;

use hybrid_array::Array;
use hybrid_array::typenum::Unsigned;

use crate::error::QueryError;
use crate::query_result::QueryResult;
use crate::raw::{ExecStatusType, PQresStatus, RawQueryResult};

impl<T> Iterator for QueryReceiver<T>
where
    for<'a> T: QueryResult<'a>,
{
    type Item = Result<T, QueryError>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.current_raw_query_result {
            None => (0, None),
            Some(_) => (self.current_chunk_row_total - self.current_chunk_row, None),
        }
    }

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_raw_query_result.take() {
            // This means its time to retrieve a fresh chunk.
            // Every time we retrieve a fresh chunk we check to make sure it's well formed.
            None => match self.recv.recv() {
                Err(e) => Some(Err(e.into())),
                Ok(r) => {
                    self.current_chunk_row = 0;

                    // Check the result status, and early return error error if its not good.
                    let status: ExecStatusType = r.PQresultStatus();
                    if !matches!(
                        status,
                        ExecStatusType::PGRES_COMMAND_OK
                            | ExecStatusType::PGRES_TUPLES_OK
                            | ExecStatusType::PGRES_SINGLE_TUPLE
                            | ExecStatusType::PGRES_TUPLES_CHUNK
                    ) {
                        self.current_raw_query_result = Some(r);
                        return Some(Err(QueryError::ResultStatusError {
                            status: PQresStatus(status),
                            query: self.query.clone(),
                        }));
                    }

                    // Check the column count, and early return error if it's lower than the expected number of columns.
                    let columns: usize = r.PQnfields();
                    if columns < <T::Columns as Unsigned>::to_usize() {
                        self.current_raw_query_result = Some(r);
                        return Some(Err(QueryError::InsufficientColumnsError {
                            query: self.query.clone(),
                            expected: <T::Columns as Unsigned>::to_usize(),
                            found: columns,
                        }));
                    }

                    // Check the field names returned, and early return error if they're not correct.
                    for column_number in 0..T::COLUMN_NAMES.len() {
                        let expected_column_name: &'static str = T::COLUMN_NAMES[column_number];
                        let actual_column_name: String = r.PQfname(column_number);
                        if expected_column_name != actual_column_name {
                            self.current_raw_query_result = Some(r);
                            return Some(Err(QueryError::ColumnNameMismatchError {
                                query: self.query.clone(),
                                column_number,
                                expected: expected_column_name,
                                found: actual_column_name,
                            }));
                        }
                    }

                    self.current_chunk_row_total = r.PQntuples();
                    // According to the docs this is the signal that no more rows will be sent:
                    // After the last row, or immediately if the query returns zero rows,
                    // a zero-row object with status PGRES_TUPLES_OK is returned;
                    // this is the signal that no more rows will arrive.
                    if self.current_chunk_row_total == 0 {
                        self.current_raw_query_result = Some(r);
                        None
                    } else {
                        let data: Array<Option<&[u8]>, T::Columns> =
                            Array::from_fn(|column| r.fetch_cell(self.current_chunk_row, column));
                        self.current_chunk_row += 1;
                        let result = match T::try_from(data) {
                            Ok(result) => Ok(result),
                            Err(e) => Err(QueryError::QueryDataError {
                                e,
                                query: self.query.clone(),
                            }),
                        };
                        if self.current_chunk_row < self.current_chunk_row_total {
                            self.current_raw_query_result = Some(r);
                        }
                        Some(result)
                    }
                }
            },
            Some(r) => {
                if self.current_chunk_row_total == 0 {
                    self.current_raw_query_result = Some(r);
                    None
                } else {
                    let data: Array<Option<&[u8]>, T::Columns> =
                        Array::from_fn(|column| r.fetch_cell(self.current_chunk_row, column));
                    self.current_chunk_row += 1;
                    let result = match T::try_from(data) {
                        Ok(result) => Ok(result),
                        Err(e) => Err(QueryError::QueryDataError {
                            e,
                            query: self.query.clone(),
                        }),
                    };
                    if self.current_chunk_row < self.current_chunk_row_total {
                        self.current_raw_query_result = Some(r);
                    }
                    Some(result)
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
    pub(crate) recv: Receiver<RawQueryResult>,
    pub(crate) phantom: std::marker::PhantomData<T>,
    pub(crate) current_raw_query_result: Option<RawQueryResult>,
    pub(crate) current_chunk_row: usize,
    pub(crate) current_chunk_row_total: usize,
}

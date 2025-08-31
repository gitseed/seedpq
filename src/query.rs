use std::option::Option;
use std::sync::mpsc::Receiver;

use hybrid_array::{Array, ArraySize};

use crate::connection_raw::SendableQueryResult;
use crate::query_error::QueryDataError;
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
    type Item = Result<T, QueryDataError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_raw_query_result {
            Some(_) => None, // TODO: Currently just returns one row ever...
            None => {
                let r: &mut RawQueryResult = self
                    .current_raw_query_result
                    .insert(self.recv.recv().unwrap().into());
                let thing: Array<Option<&[u8]>, T::Columns> =
                    Array::from_fn(|column| r.fetch_cell(0, column));
                match T::try_from(thing) {
                    Ok(result) => Some(Ok(result)),
                    Err(e) => Some(Err(e)),
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
}

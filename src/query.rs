use std::option::Option;
use std::sync::mpsc::Receiver;

use hybrid_array::{Array, ArraySize};

use crate::connection_raw::SendableQueryResult;
use crate::query_raw::RawQueryResult;

pub trait QueryResult<'a>: From<Array<Option<&'a [u8]>, Self::Columns>> {
    type Columns: ArraySize;
}

impl<'a, T: QueryResult<'a>> QueryReceiver<T> {
    pub fn fetch_one(&'a self) -> T {
        Array::from_fn(|column| self.query_result_temp.fetch_cell(0, column)).into()
    }
}

/// Receives results from a single query, from the database connection thread.
/// The methods of this struct may block.
#[derive(Debug)]
pub struct QueryReceiver<T> {
    pub(crate) query: String,
    pub(crate) recv: Receiver<SendableQueryResult>,
    pub(crate) phantom: std::marker::PhantomData<T>,
    pub(crate) query_result_temp: RawQueryResult,
}

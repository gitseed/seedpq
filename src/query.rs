use std::sync::mpsc::Receiver;

use crate::connection_error::ConnectionError;
use crate::libpq;
use crate::query_error::QueryError;
use crate::query_raw::RawQueryResult;

pub struct QueryResult<const N: usize> {
    result: *mut libpq::PGresult,
}

/// Receives results from a single query, from the database connection thread.
/// The methods of this struct may block.
pub struct QueryReceiver<const N: usize> {
    pub(crate) query: String,
    pub(crate) recv: Receiver<RawQueryResult>,
}

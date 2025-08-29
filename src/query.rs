use std::sync::mpsc::Receiver;

use crate::libpq;
use crate::query_error::QueryError;
use crate::query_raw::RawQueryResult;

pub struct QueryResult<const N: usize> {
    result: *mut libpq::PGresult,
}

/// Receives the results of queries sent to the connection.
/// The methods of this struct will block.
pub struct QueryReceiver {
    pub(crate) recv: Receiver<RawQueryResult>,
}

impl QueryReceiver {
    pub fn get<const N: usize>(&self) -> Result<QueryResult<N>, QueryError> {
        match self.recv.recv() {
            Ok(r) => r.unwrap(),
            Err(e) => Err(QueryError::RecvError(e)),
        }
    }
}

use std::sync::mpsc::Receiver;

use crate::connection_error::ConnectionError;
use crate::query::QueryReceiver;
use crate::query_error::QueryError;
use crate::query_raw::RawQueryResult;
use crate::query_recv_error::QueriesReceiverError;

/// Receives a QueryReceiver.
/// The methods of this struct may block.
pub struct QueriesReceiver {
    pub(crate) recv: Receiver<(String, Result<Receiver<RawQueryResult>, ConnectionError>)>,
}

impl QueriesReceiver {
    pub fn get<const N: usize>(&self) -> Result<QueryReceiver<N>, QueriesReceiverError> {
        match self.recv.recv() {
            Ok((query, r)) => match r {
                Ok(recv) => Ok(QueryReceiver { query, recv }),
                Err(e) => Err(QueriesReceiverError::ConnectionError { query, e }),
            },
            Err(e) => Err(QueriesReceiverError::RecvError(e)),
        }
    }
}

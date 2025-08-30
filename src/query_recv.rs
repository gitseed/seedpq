use std::sync::mpsc::Receiver;

use crate::connection_error::ConnectionError;
use crate::connection_raw::SendableQueryResult;
use crate::query::QueryReceiver;
use crate::query_recv_error::QueriesReceiverError;

/// The receiving end of a database connection, that receives queries.
pub struct QueriesReceiver {
    pub(crate) recv: Receiver<(
        String,
        Result<Receiver<SendableQueryResult>, ConnectionError>,
    )>,
}

impl QueriesReceiver {
    /// Gets a QueryReceiver<T> that can be used to retrieve the results from a single query.
    /// This method blocks.
    pub fn get<T>(&self) -> Result<QueryReceiver<T>, QueriesReceiverError> {
        match self.recv.recv() {
            Ok((query, r)) => match r {
                Ok(recv) => {
                    let temp = recv.recv().unwrap().unwrap();
                    Ok(QueryReceiver {
                        query,
                        recv,
                        phantom: std::marker::PhantomData,
                        query_result_temp: temp,
                    })
                }
                Err(e) => Err(QueriesReceiverError::ConnectionError { query, e }),
            },
            Err(e) => Err(QueriesReceiverError::RecvError(e)),
        }
    }
}

use std::sync::mpsc;

use thiserror::Error;

use crate::connection_error::ConnectionError;

#[derive(Error, Debug)]
pub enum QueriesReceiverError {
    #[error("postgres connection thread unexpectedly hung up")]
    RecvError(#[from] mpsc::RecvError),
    #[error("connection error while executing {query}\n{e}")]
    ConnectionError { query: String, e: ConnectionError },
}

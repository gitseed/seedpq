use thiserror::Error;

use crate::request::PostgresRequest;

#[derive(Error, Debug)]
pub enum RequestSenderError {
    #[error("postgres connection thread unexpectedly hung up while sending request")]
    RecvError(#[from] std::sync::mpsc::SendError<PostgresRequest>),
}

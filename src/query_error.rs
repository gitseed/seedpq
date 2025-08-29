use std::sync::mpsc;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("insufficent columns returned by query {query}\nexpected {expected}, found {found})")]
    InsufficientColumns {
        query: String,
        expected: usize,
        found: usize,
    },
    #[error("connection error while executing {query}\n{msg}")]
    ConnectionError { query: String, msg: String },
    #[error("postgres connection thread unexpectedly hung up")]
    RecvError(#[from] mpsc::RecvError),
}

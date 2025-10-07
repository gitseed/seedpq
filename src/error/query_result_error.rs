use std::error::Error;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("while converting to {t} in column {column} got error:\n{e}")]
pub struct QueryResultError {
    pub e: Box<dyn Error + Send + Sync>,
    pub t: &'static str,
    pub column: usize,
}

#[derive(Error, Debug)]
#[error("query result was not empty")]
pub struct NotEmpty;

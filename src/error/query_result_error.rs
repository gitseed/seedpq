use std::error::Error;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("while converting to {t} in {column} got the following error:\n{e}")]
pub struct QueryResultError {
    pub e: Box<dyn Error>,
    pub t: &'static str,
    pub column: usize,
}

use thiserror::Error;

#[derive(Error, Debug)]
#[error("cannot convert a NULL into a non nullable type")]
pub struct UnexpectedNullError;

#[derive(Error, Debug)]
#[error("expected a boolean but the byte is not 0 or 1 it is instead {actual}")]
pub struct BadBooleanError {
    pub actual: u8,
}

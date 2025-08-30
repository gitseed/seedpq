use std::sync::mpsc;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("connection is bad")]
    ConnectionBad,
}

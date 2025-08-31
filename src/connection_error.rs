use thiserror::Error;

use crate::connection_raw::ConnStatusType;

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("connection is bad, has status: {status:?}\n{msg}")]
    ConnectionBad { status: ConnStatusType, msg: String },
}

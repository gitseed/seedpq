use thiserror::Error;

use crate::connection_raw::ConnStatusType;

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("connection is bad, has status: {status:?}\n{msg}")]
    ConnectionBad { status: ConnStatusType, msg: String },
    #[error("query wasn't dispatched successfully, connection has status: {status:?}\n{msg}")]
    QueryUnsuccessfullyDispatched { status: ConnStatusType, msg: String },
    #[error(
        "failed setting chunked rows mode, likely because your postgres database version is too old, connection has status: {status:?}\n{msg}"
    )]
    FailedSettingChunkedRowsMode { status: ConnStatusType, msg: String },
}

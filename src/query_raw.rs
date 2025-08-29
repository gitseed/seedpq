use crate::libpq;

use crate::query::QueryResult;
use crate::query_error::QueryError;

/// Private query result type.
/// Only good for unwrapping into a Result<QueryResult, QueryError>.
pub(crate) struct RawQueryResult {
    pub(crate) result: Option<*mut libpq::PGresult>,
    pub(crate) connection_error_message: Option<String>,
    pub(crate) query: String,
}

// SAFETY: We send the RawQueryResult to the recieving end which unwraps into a Result<QueryResult, QueryError>.
// The underlying pointer is never used until during or after the RawQueryResult is unwrapped.
// Unwrap consumes the RawQueryResult, so it can only be run once.
// String and Option<String> are safe to Send.
// The public type QueryResult is !Send, due to having a *mut and not being marked as unsafe impl Send.
unsafe impl Send for RawQueryResult {}

impl RawQueryResult {
    pub(crate) fn unwrap<const N: usize>(self) -> Result<QueryResult<N>, QueryError> {
        match self.connection_error_message {
            Some(s) => Err(QueryError::ConnectionError {
                query: self.query,
                msg: s,
            }),
            None => {
                todo!()
            }
        }
    }
}

use std::sync::mpsc::Sender;

use crate::request_error::RequestSenderError;

/// Send queries or requests for info to the connection.
/// The connection will send the results back in the same order of the requests.
/// The methods of this struct do not block.
/// Dropping this will cause the postgres connection to close.
pub struct RequestSender {
    pub(crate) send: Sender<PostgresRequest>,
}

/// The different types of requests that can be sent to postgres through a RequestSender.
pub enum PostgresRequest {
    Query {
        query: String,
        chunk_size: std::ffi::c_int,
    },
}

impl RequestSender {
    /// Sends the query string to postgres to be executed.
    /// Whether the execution is successful or not, the result will be sent to the QueryReceiver.
    /// If None is specified for chunk size, it will use a default chunk size.
    pub fn exec(&self, query: &str, chunk_size: Option<usize>) -> Result<(), RequestSenderError> {
        const DEFAULT_CHUNK_SIZE: std::ffi::c_int = 100;

        let chunk_size: std::ffi::c_int = match chunk_size {
            None => DEFAULT_CHUNK_SIZE,
            Some(s) => s as std::ffi::c_int,
        };

        match self.send.send(PostgresRequest::Query {
            query: query.to_owned(),
            chunk_size,
        }) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}

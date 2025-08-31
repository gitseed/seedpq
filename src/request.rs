use std::sync::mpsc::Sender;

/// Send queries or requests for info to the connection.
/// The connection will send the results back in the same order of the requests.
/// The methods of this struct do not block.
/// Dropping this will cause the postgres connection to close.
pub struct RequestSender {
    pub(crate) send: Sender<PostgresRequest>,
}

/// The different types of requests that can be sent to postgres through a RequestSender.
pub(crate) enum PostgresRequest {
    Query(String),
}

impl RequestSender {
    /// Sends the query string to postgres to be executed.
    /// Whether the execution is successful or not, the result will be sent to the QueryReceiver.
    /// Currently panics if the postgres thread hangs up, because I'm lazy.
    /// todo: remove panic
    pub fn exec(&self, query: &str) {
        self.send
            .send(PostgresRequest::Query(String::from(query)))
            .unwrap()
    }
}

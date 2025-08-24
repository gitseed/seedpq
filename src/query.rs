use std::sync::mpsc::Receiver;

pub struct QueryResult {}

pub enum QueryError {}

/// Receives the results of queries sent to the connection.
/// The methods of this struct will block.
pub struct QueryReceiver {
    pub(crate) recv: Receiver<Result<QueryResult, QueryError>>,
}

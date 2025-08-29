use std::sync::mpsc::Receiver;

use thiserror::Error;

pub struct QueryResult {}

#[derive(Error, Debug)]
pub enum QueryError {}

/// Receives the results of queries sent to the connection.
/// The methods of this struct will block.
pub struct QueryReceiver {
    pub(crate) recv: Receiver<Result<QueryResult, QueryError>>,
}

impl QueryReceiver {
    pub fn get(&self) -> Result<QueryResult, QueryError> {
        self.recv.recv().unwrap()
    }
}

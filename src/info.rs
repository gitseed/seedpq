use std::sync::mpsc::Receiver;

/// Info that can be received from Postgres, that is something other than a query result.
/// For example "the current in-transaction status of the server."
/// These are generally returned by functions in: 32.2. Connection Status Functions.
#[non_exhaustive]
pub enum Info {}

/// Receives the results of requests for info sent to the connection.
/// For example for checking the transaction status.
/// The methods of this struct will block.
#[allow(dead_code)]
pub struct InfoReceiver {
    pub(crate) recv: Receiver<Info>,
}

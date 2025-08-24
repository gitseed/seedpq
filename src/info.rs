/// Info that can be recieved from Postgres, that is something other than a query result.
/// For example "the current in-transaction status of the server."
/// These are generally returned by functions in: 32.2. Connection Status Functions.
#[non_exhaustive]
pub enum Info {}

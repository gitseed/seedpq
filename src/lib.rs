//! Library entry point for seedpq. Just pub mod statements for now.

// Unsafe modules.
mod connection_raw;
mod libpq;
mod query_raw;

// Safe modules.
mod connection;
mod error;
mod info;
mod notice;
mod queries_recv;
mod query;
mod request;
mod request_error;

// Public library interfaces, make sure everything here is documented.
pub use error::ConnectionError;
pub use error::QueriesReceiverError;
pub use error::QueryDataError;
pub use error::QueryError;

pub use connection::connect;

pub use query::EmptyResult;
pub use query::QueryReceiver;
pub use query::QueryResult;

// Re export hybrid_array, so that it can be used in derive macros.
pub use hybrid_array;

//! Library entry point for seedpq. Just pub mod statements for now.

// Unsafe submodule.
mod raw;

// Safe modules.
mod connection;
mod empty_result;
mod error;
mod info;
mod notice;
mod postgres_data;
mod queries_recv;
mod query_recv;
mod query_result;
mod request;
mod request_error;

// Public library interfaces, make sure everything here is documented.
pub use error::ConnectionError;
pub use error::QueriesReceiverError;
pub use error::QueryDataError;
pub use error::QueryError;

pub use connection::connect;

pub use empty_result::EmptyResult;
pub use postgres_data::PostgresData;
pub use query_recv::QueryReceiver;
pub use query_result::QueryResult;

// Re export hybrid_array, so that it can be used in derive macros.
pub use hybrid_array;

// Re export derive macro...
pub use derive::QueryResult;

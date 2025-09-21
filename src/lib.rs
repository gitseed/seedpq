//! Library entry point for seedpq. Just pub mod statements for now.
// #![allow(warnings)]
#![allow(dead_code)]

// Unsafe modules.
mod connection_raw;
mod libpq;
mod query_raw;

// Safe modules.
mod connection;
mod connection_error;
mod info;
mod notice;
mod query;
mod query_error;
mod query_recv;
mod query_recv_error;
mod request;
mod request_error;

// Public library interfaces, make sure everything here is doc
pub use connection::connect;
pub use query::EmptyResult;
pub use query::QueryReceiver;
pub use query::QueryResult;

pub mod error {
    pub use super::query_error::QueryDataError;
    pub use super::query_error::QueryError;
}

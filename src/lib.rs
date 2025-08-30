//! Library entry point for seedpq. Just pub mod statements for now.
#![allow(warnings)]

/// The underlying libpq bit twiddling is kept away from library users.
mod connection_raw;
mod libpq;

pub mod connection;
pub mod connection_error;
pub mod info;
pub mod notice;
pub mod query;
pub mod query_error;
pub mod query_raw;
pub mod query_recv;
pub mod query_recv_error;
pub mod request;

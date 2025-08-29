//! Library entry point for seedpq. Just pub mod statements for now.
#![allow(warnings)]

mod libpq;
/// The underlying libpq bit twiddling is kept away from library users.
mod raw_connection;

pub mod connection;
pub mod info;
pub mod notice;
pub mod query;
pub mod query_error;
pub mod query_raw;
pub mod request;

use std::sync::mpsc;

use thiserror::Error;

/// An error obtained while getting a QueryResult from a QueryReceiver.
/// This almost always knows of the text of the query that was run as well as the row.
/// The one exception, is when the database thread hangs up unexpectedly.
#[derive(Error, Debug)]
pub enum QueryError {
    #[error("postgres connection thread unexpectedly hung up")]
    RecvError(#[from] mpsc::RecvError),
    #[error(
        "insufficient columns returned by query, expected {expected} found {found}, query:\n{query}"
    )]
    InsufficientColumnsError {
        query: String,
        expected: usize,
        found: usize,
    },
    #[error("connection error while executing query:\n{query}\n{msg}")]
    ConnectionError { query: String, msg: String },
    #[error("error converting query results into a rust type, query:\n{query}")]
    QueryDataError {
        #[source]
        e: QueryDataError,
        query: String,
    },
}

/// An error obtained while converting from a QueryResult<T> into a T.
/// This doesn't know its query text, but it does know its column and it will be specific to a single column.
/// It also knows the type it's trying to decode the row into.
/// Should aways be upcasted into a QueryError so that it can be associated with query text.
#[derive(Error, Debug)]
pub enum QueryDataError {
    #[error("while converting to {t}, textual result in {column} wasn't valid utf8")]
    Utf8Error {
        #[source]
        e: std::str::Utf8Error,
        t: &'static str,
        column: usize,
    },
    #[error("while converting to {t}, column {column} is not nullable, but a null was found")]
    UnexpectedNullError { t: &'static str, column: usize },
    #[error(
        "while converting column {column} to {t}, expected {numsize} bytes, got {slicesize} bytes"
    )]
    WrongSizeNumericError {
        t: &'static str,
        e: std::array::TryFromSliceError,
        column: usize,
        numsize: usize,
        slicesize: usize,
    },
}

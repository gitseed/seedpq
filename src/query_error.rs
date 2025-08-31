use std::sync::mpsc;

use thiserror::Error;

/// An error obtained while getting a QueryResult from a QueryReceiver.
/// This almost always knows of the text of the query that was run as well as the row.
/// The one exception, is when the database thread hangs up unexpectedly.
#[derive(Error, Debug)]
pub enum QueryError {
    #[error("postgres connection thread unexpectedly hung up")]
    RecvError(#[from] mpsc::RecvError),
    #[error("insufficent columns returned by query {query}\nexpected {expected}, found {found})")]
    InsufficientColumns {
        query: String,
        expected: usize,
        found: usize,
    },
    #[error("connection error while executing {query}\n{msg}")]
    ConnectionError { query: String, msg: String },
    #[error("error converting the results of {query} into a rust type")]
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
}

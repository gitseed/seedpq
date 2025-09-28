use std::sync::mpsc;

use thiserror::Error;

use super::query_result_error::QueryResultError;

/// An error obtained while getting a QueryResult from a QueryReceiver.
/// This almost always knows of the text of the query that was run as well as the row.
/// The one exception, is when the database thread hangs up unexpectedly.
#[derive(Error, Debug)]
pub enum QueryError {
    #[error("postgres connection thread unexpectedly hung up")]
    RecvError(#[from] mpsc::RecvError),
    #[error(
        "insufficient columns in query result, expected {expected} found {found}, query:\n{query}"
    )]
    InsufficientColumnsError {
        query: String,
        expected: usize,
        found: usize,
    },
    #[error(
        "column name mismatch in query result, for column {column_number} expected label {expected} found label {found}, query:\n{query}"
    )]
    ColumnNameMismatchError {
        query: String,
        column_number: usize,
        expected: &'static str,
        found: String,
    },
    #[error("connection error while executing query:\n{query}\n{msg}")]
    ConnectionError { query: String, msg: String },
    #[error("error: {e}\nquery:\n{query}")]
    QueryDataError {
        #[source]
        e: QueryResultError,
        query: String,
    },
    #[error("PGresult had bad status {status}, for query:\n{query}")]
    ResultStatusError { status: &'static str, query: String },
    #[error("tried to fetch one row, but out of rows, for query:\n{query}")]
    OutOfRowsError { query: String },
}

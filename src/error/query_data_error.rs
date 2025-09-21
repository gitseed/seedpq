use thiserror::Error;

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

use crate::libpq;

use crate::query::PartialQueryResult;
use crate::query_error::QueryError;

/// A private struct containing the raw C pointer to a PGresult.
/// Dropping this will call
pub(crate) struct RawQueryResult {
    /// Private! We only want SendableQueryResult being constructed by wrapper functions that would return *mut PGresult
    pub(crate) result: *mut libpq::PGresult,
}

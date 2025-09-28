use crate::PostgresRow;
use crate::query_result::QueryResult;

use crate::error::NotEmpty;
use crate::error::QueryResultError;

use hybrid_array::Array;
use hybrid_array::typenum::U0;

impl QueryResult<'_> for () {
    type Columns = U0;
    const COLUMN_NAMES: Option<Array<&'static str, Self::Columns>> = Some(Array([]));
}

impl TryFrom<PostgresRow<'_, U0>> for () {
    type Error = QueryResultError;

    /// Always fails, because empty query results should have zero rows.
    fn try_from(_: PostgresRow<'_, U0>) -> Result<Self, Self::Error> {
        Err(QueryResultError {
            e: Box::new(NotEmpty),
            t: std::any::type_name::<()>(),
            column: 0,
        })
    }
}

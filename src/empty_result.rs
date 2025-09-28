use crate::PostgresRow;
use crate::error::QueryResultError;
use crate::query_result::QueryResult;

use hybrid_array::Array;
use hybrid_array::typenum::U0;

pub struct EmptyResult;

impl QueryResult<'_> for EmptyResult {
    type Columns = U0;
    const COLUMN_NAMES: Option<Array<&'static str, Self::Columns>> = Some(Array([]));
}

impl TryFrom<PostgresRow<'_, U0>> for EmptyResult {
    type Error = QueryResultError;

    fn try_from(_: PostgresRow<'_, U0>) -> Result<Self, Self::Error> {
        unreachable!()
    }
}

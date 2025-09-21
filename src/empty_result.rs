use crate::error::QueryDataError;
use crate::query_result::QueryResult;

use hybrid_array::Array;
use hybrid_array::typenum::U0;

pub struct EmptyResult;

impl QueryResult<'_> for EmptyResult {
    type Columns = U0;
    const COLUMN_NAMES: Array<&'static str, Self::Columns> = Array([]);
}

impl TryFrom<Array<Option<&[u8]>, U0>> for EmptyResult {
    type Error = QueryDataError;

    fn try_from(_: Array<Option<&[u8]>, U0>) -> Result<Self, Self::Error> {
        unreachable!()
    }
}

use hybrid_array::{Array, ArraySize};

use crate::PostgresRow;

use crate::error::QueryResultError;

pub trait QueryResult<'a>:
    TryFrom<PostgresRow<'a, Self::Columns>, Error = QueryResultError>
{
    type Columns: ArraySize;
    const COLUMN_NAMES: Option<Array<&'static str, Self::Columns>>;
}

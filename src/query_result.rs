use hybrid_array::{Array, ArraySize};

use crate::error::QueryResultError;
use crate::postgres_data::PostgresData;

pub trait QueryResult<'a>:
    TryFrom<Array<PostgresData<'a>, Self::Columns>, Error = QueryResultError>
{
    type Columns: ArraySize;
    const COLUMN_NAMES: Array<&'static str, Self::Columns>;
}

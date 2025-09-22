use hybrid_array::{Array, ArraySize};

use crate::error::QueryDataError;
use crate::postgres_data::PostgresData;

pub trait QueryResult<'a>:
    TryFrom<Array<PostgresData<'a>, Self::Columns>, Error = QueryDataError>
{
    type Columns: ArraySize;
    const COLUMN_NAMES: Array<&'static str, Self::Columns>;
}

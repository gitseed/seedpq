use std::option::Option;

use hybrid_array::{Array, ArraySize};

use crate::error::QueryDataError;

pub trait QueryResult<'a>:
    TryFrom<Array<Option<&'a [u8]>, Self::Columns>, Error = QueryDataError>
{
    type Columns: ArraySize;
    const COLUMN_NAMES: Array<&'static str, Self::Columns>;
}

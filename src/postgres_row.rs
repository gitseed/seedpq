use crate::PostgresData;
use hybrid_array::{Array, ArraySize};

#[derive(Debug)]
pub struct PostgresRow<'a, N: ArraySize>(pub Array<PostgresData<'a>, N>);

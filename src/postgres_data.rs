#[derive(Copy, Clone, Debug)]
pub struct PostgresData<'a>(pub Option<&'a [u8]>);

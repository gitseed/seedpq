#[derive(Copy, Clone, Debug)]
pub struct PostgresData<'a>(pub Option<&'a [u8]>);

impl<'a> Into<Option<&'a [u8]>> for PostgresData<'a> {
    fn into(self) -> Option<&'a [u8]> {
        self.0
    }
}

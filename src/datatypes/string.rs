use crate::PostgresData;

use crate::error::UnexpectedNullError;

use super::single_value_result::single_value_result;

use std::error::Error;

impl TryInto<String> for PostgresData<'_> {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<String, Box<dyn Error>> {
        match self.0 {
            None => Err(Box::new(UnexpectedNullError)),
            Some(s) => Ok(str::from_utf8(s)?.to_owned()),
        }
    }
}
single_value_result!(String);

impl TryInto<Option<String>> for PostgresData<'_> {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<Option<String>, Box<dyn Error>> {
        match self.0 {
            None => Ok(None),
            Some(s) => Ok(Some(str::from_utf8(s)?.to_owned())),
        }
    }
}
single_value_result!(Option<String>);

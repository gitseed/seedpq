use super::single_value_result::single_value_result;
use crate::PostgresData;
use crate::error::UnexpectedNullError;
use std::error::Error;

impl TryFrom<PostgresData<'_>> for String {
    type Error = Box<dyn Error + Send + Sync>;
    fn try_from(value: PostgresData) -> Result<Self, Self::Error> {
        match value.0 {
            None => Err(Box::new(UnexpectedNullError)),
            Some(s) => Ok(str::from_utf8(s)?.to_owned()),
        }
    }
}
single_value_result!(String);

impl TryFrom<PostgresData<'_>> for Option<String> {
    type Error = Box<dyn Error + Send + Sync>;
    fn try_from(value: PostgresData) -> Result<Self, Self::Error> {
        match value.0 {
            None => Ok(None),
            Some(s) => Ok(Some(str::from_utf8(s)?.to_owned())),
        }
    }
}
single_value_result!(Option<String>);

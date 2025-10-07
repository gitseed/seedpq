use super::single_value_result::single_value_result;
use crate::error::BadBooleanError;
use crate::error::UnexpectedNullError;
use crate::postgres_data::PostgresData;

use std::error::Error;

impl TryFrom<PostgresData<'_>> for bool {
    type Error = Box<dyn Error + Send + Sync>;
    fn try_from(value: PostgresData) -> Result<Self, Self::Error> {
        match value.0 {
            None => Err(Box::new(UnexpectedNullError)),
            Some(s) => match <[u8; 1]>::try_from(s) {
                Err(e) => Err(Box::new(e)),
                Ok([n]) => match n {
                    0 => Ok(false),
                    1 => Ok(true),
                    _ => Err(Box::new(BadBooleanError { actual: n })),
                },
            },
        }
    }
}
single_value_result!(bool);

impl TryFrom<PostgresData<'_>> for Option<bool> {
    type Error = Box<dyn Error + Send + Sync>;
    fn try_from(value: PostgresData) -> Result<Self, Self::Error> {
        match value.0 {
            None => Ok(None),
            Some(s) => match <[u8; 1]>::try_from(s) {
                Err(e) => Err(Box::new(e)),
                Ok(arr) => match arr[0] {
                    0 => Ok(Some(false)),
                    1 => Ok(Some(true)),
                    _ => Err(Box::new(BadBooleanError { actual: arr[0] })),
                },
            },
        }
    }
}
single_value_result!(Option<bool>);

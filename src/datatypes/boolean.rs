use crate::error::BadBooleanError;
use crate::error::UnexpectedNullError;
use crate::postgres_data::PostgresData;

use std::error::Error;

impl TryInto<bool> for PostgresData<'_> {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<bool, Box<dyn Error>> {
        match self.0 {
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

impl TryInto<Option<bool>> for PostgresData<'_> {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<Option<bool>, Box<dyn Error>> {
        match self.0 {
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

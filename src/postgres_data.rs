use crate::error::BadBooleanError;
use crate::error::UnexpectedNullError;

use std::error::Error;

pub struct PostgresData<'a>(pub Option<&'a [u8]>);

impl TryInto<String> for PostgresData<'_> {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<String, Box<dyn Error>> {
        match self.0 {
            None => Err(Box::new(UnexpectedNullError)),
            Some(s) => Ok(str::from_utf8(s)?.to_owned()),
        }
    }
}

impl TryInto<Option<String>> for PostgresData<'_> {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<Option<String>, Box<dyn Error>> {
        match self.0 {
            None => Ok(None),
            Some(s) => Ok(Some(str::from_utf8(s)?.to_owned())),
        }
    }
}

impl TryInto<bool> for PostgresData<'_> {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<bool, Box<dyn Error>> {
        match self.0 {
            None => Err(Box::new(UnexpectedNullError)),
            Some(s) => match <[u8; 1]>::try_from(s) {
                Err(e) => Err(Box::new(e)),
                Ok(arr) => match arr[0] {
                    0 => Ok(false),
                    1 => Ok(true),
                    _ => Err(Box::new(BadBooleanError { actual: arr[0] })),
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

macro_rules! numeric_impl {
    ($num_type:ty) => {
        impl TryInto<$num_type> for PostgresData<'_> {
            type Error = Box<dyn Error>;

            fn try_into(self) -> Result<$num_type, Box<dyn Error>> {
                match self.0 {
                    None => Err(Box::new(UnexpectedNullError)),
                    Some(s) => match <[u8; size_of::<$num_type>()]>::try_from(s) {
                        Err(e) => Err(Box::new(e)),
                        Ok(arr) => Ok(<$num_type>::from_be_bytes(arr)),
                    },
                }
            }
        }
    };
}

macro_rules! nullable_numeric_impl {
    ($num_type:ty) => {
        impl TryInto<Option<$num_type>> for PostgresData<'_> {
            type Error = Box<dyn Error>;

            fn try_into(self) -> Result<Option<$num_type>, Box<dyn Error>> {
                match self.0 {
                    None => Ok(None),
                    Some(s) => match <[u8; size_of::<$num_type>()]>::try_from(s) {
                        Err(e) => Err(Box::new(e)),
                        Ok(arr) => Ok(Some(<$num_type>::from_be_bytes(arr))),
                    },
                }
            }
        }
    };
}

numeric_impl!(usize);
numeric_impl!(u8);
numeric_impl!(u16);
numeric_impl!(u32);
numeric_impl!(u64);
numeric_impl!(u128);
numeric_impl!(i8);
numeric_impl!(i16);
numeric_impl!(i32);
numeric_impl!(i64);
numeric_impl!(i128);
numeric_impl!(f32);
numeric_impl!(f64);

nullable_numeric_impl!(usize);
nullable_numeric_impl!(u8);
nullable_numeric_impl!(u16);
nullable_numeric_impl!(u32);
nullable_numeric_impl!(u64);
nullable_numeric_impl!(u128);
nullable_numeric_impl!(i8);
nullable_numeric_impl!(i16);
nullable_numeric_impl!(i32);
nullable_numeric_impl!(i64);
nullable_numeric_impl!(i128);
nullable_numeric_impl!(f32);
nullable_numeric_impl!(f64);

use super::single_value_result::single_value_result;
use crate::error::UnexpectedNullError;
use crate::postgres_data::PostgresData;

use std::error::Error;

macro_rules! numeric_impl {
    ($num_type:ty) => {
        impl TryFrom<PostgresData<'_>> for $num_type {
            type Error = Box<dyn Error + Send + Sync>;
            fn try_from(value: PostgresData<'_>) -> Result<Self, Self::Error> {
                match value.0 {
                    None => Err(Box::new(UnexpectedNullError)),
                    Some(s) => match <[u8; size_of::<Self>()]>::try_from(s) {
                        Err(e) => Err(Box::new(e)),
                        Ok(arr) => Ok(Self::from_be_bytes(arr)),
                    },
                }
            }
        }
    };
}

macro_rules! nullable_numeric_impl {
    ($num_type:ty) => {
        impl TryFrom<PostgresData<'_>> for Option<$num_type> {
            type Error = Box<dyn Error + Send + Sync>;
            fn try_from(value: PostgresData<'_>) -> Result<Self, Self::Error> {
                match value.0 {
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

single_value_result!(usize);
single_value_result!(u8);
single_value_result!(u16);
single_value_result!(u32);
single_value_result!(u64);
single_value_result!(u128);
single_value_result!(i8);
single_value_result!(i16);
single_value_result!(i32);
single_value_result!(i64);
single_value_result!(i128);
single_value_result!(f32);
single_value_result!(f64);

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

single_value_result!(Option<usize>);
single_value_result!(Option<u8>);
single_value_result!(Option<u16>);
single_value_result!(Option<u32>);
single_value_result!(Option<u64>);
single_value_result!(Option<u128>);
single_value_result!(Option<i8>);
single_value_result!(Option<i16>);
single_value_result!(Option<i32>);
single_value_result!(Option<i64>);
single_value_result!(Option<i128>);
single_value_result!(Option<f32>);
single_value_result!(Option<f64>);

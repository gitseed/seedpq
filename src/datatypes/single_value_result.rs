macro_rules! single_value_result {
    ($t:ty) => {
        impl crate::QueryResult<'_> for $t {
            type Columns = ::hybrid_array::typenum::U1;
            const COLUMN_NAMES: Option<::hybrid_array::Array<&'static str, Self::Columns>> = None;
        }

        impl TryFrom<crate::PostgresRow<'_, ::hybrid_array::typenum::U1>> for $t {
            type Error = crate::QueryResultError;

            fn try_from(
                value: crate::PostgresRow<::hybrid_array::typenum::U1>,
            ) -> Result<Self, Self::Error> {
                let value: Result<$t, Box<dyn Error>> = value.0[0].try_into();
                match value {
                    Ok(result) => Ok(result),
                    Err(e) => Err(crate::QueryResultError {
                        e,
                        t: ::std::any::type_name::<Self>(),
                        column: 0,
                    }),
                }
            }
        }
    };
}

pub(crate) use single_value_result;

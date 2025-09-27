#![allow(unused_imports)]

use crate::PostgresData;
use crate::QueryResult;

use crate::error::QueryResultError;
use crate::error::UnexpectedNullError;

use hybrid_array::Array;
use hybrid_array::typenum::U1;

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

impl TryInto<Option<String>> for PostgresData<'_> {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<Option<String>, Box<dyn Error>> {
        match self.0 {
            None => Ok(None),
            Some(s) => Ok(Some(str::from_utf8(s)?.to_owned())),
        }
    }
}

// impl QueryResult<'_> for Option<String> {
//     type Columns = U1;
//     const COLUMN_NAMES: Option<Array<&'static str, Self::Columns>> = None;
// }

// impl TryFrom<Array<PostgresData<'_>, U1>> for Option<String> {
//     type Error = QueryResultError;

//     fn try_from(_: Array<PostgresData, U1>) -> Result<Self, Self::Error> {
//         todo!()
//     }
// }

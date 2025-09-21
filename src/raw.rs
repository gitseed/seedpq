#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
// This is the submodule that's containment for all the unsafe code.
#![allow(unsafe_code)]

mod connection_raw;
mod libpq;
mod misc;
mod query_raw;

pub(crate) use connection_raw::RawConnection;
pub(crate) use connection_raw::custom_notice_receiver;
pub(crate) use misc::PQresStatus;
pub(crate) use query_raw::RawQueryResult;

// It's safe to give people libpq types.
pub(crate) use libpq::ConnStatusType;
pub(crate) use libpq::ExecStatusType;

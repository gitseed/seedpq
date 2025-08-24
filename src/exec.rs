use std::ptr::null;

use crate::connection::{Connection, ConnectionError};
use crate::query_result::QueryResult;

use crate::libpq;

/// A pending query that can be awaited to obtain a Result<QueryResult, QueryError>.
pub struct PendingQuery<'a> {
    conn: &'a mut Connection,
}

impl Future for PendingQuery<'_> {
    type Output = QueryResult;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let raw_result: *mut libpq::pg_result = unsafe { libpq::PQgetResult(self.conn.raw()) };
        unsafe { libpq::PQgetResult(self.conn.raw()) };
        std::task::Poll::Ready(QueryResult { result: raw_result })
    }
}

impl Connection {
    #[allow(mismatched_lifetime_syntaxes)]
    pub fn exec(&mut self, query: &str) -> Result<PendingQuery, ConnectionError> {
        if !self.ok {
            return Err(self.error());
        }

        let ffi_query: std::ffi::CString = std::ffi::CString::new(query)
            .expect("postgres queries can not contain null characters");

        let sent_successfully: std::ffi::c_int = unsafe {
            libpq::PQsendQueryParams(
                self.raw(),
                ffi_query.as_ptr(),
                0,
                null(),
                null(),
                null(),
                null(),
                0,
            )
        };

        // From the docs: "1 is returned if the command was successfully dispatched and 0 if not"
        if sent_successfully != 1 {
            Err(self.error())
        } else {
            Ok(PendingQuery { conn: self })
        }
    }
}

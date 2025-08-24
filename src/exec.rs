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
        if true {
            // We can now call PQgetResult without blocking.
            // Because we're using PQsendQueryParams there should only be one pg_result followed by a nullptr.
            let raw_result: *mut libpq::pg_result = unsafe { libpq::PQgetResult(self.conn.raw()) };
            assert!(!raw_result.is_null());
            let expecting_null: *mut libpq::pg_result =
                unsafe { libpq::PQgetResult(self.conn.raw()) };
            assert!(expecting_null.is_null());
            std::task::Poll::Ready(QueryResult { result: raw_result })
        } else {
            std::task::Poll::Pending
        }
    }
}

impl Connection {
    // TODO: Fix whatever lifetime oopsie is going on here
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
                // Specify zero to obtain results in text format, or one to obtain results in binary format.
                // If you specify text format then numbers wil be sent in text form which is dumb.
                1,
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

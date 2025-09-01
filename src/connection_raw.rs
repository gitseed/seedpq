// We allow non_snake_case because it more convenient to have the impls of RawConnection directly map to libpq functions.
#![allow(non_snake_case)]

use crate::libpq;

use crate::query_raw::RawQueryResult;

use std::ptr::null;

/// Sendable query result type.
/// Used to ensure that the underlying *mut libpq::PGresult is not utilized in multiple threads.
pub(crate) struct SendableQueryResult {
    /// Private! We only want SendableQueryResult being constructed by wrapper functions that would return *mut PGresult
    result: Option<*mut libpq::PGresult>,
}

// SAFETY: We send the SendableQueryResult to the receiving end which unwraps into a RawQueryResult.
// The underlying pointer is never used until during or after the SendableQueryResult is unwrapped.
// Unwrap consumes the SendableQueryResult, so it can only be run once.
// The unwrapped type RawQueryResult is !Send, due to having a *mut and not being marked as unsafe impl Send.
unsafe impl Send for SendableQueryResult {}

// We don't want to define the from, because we don't want to give RawQueryResult visibility into SendableQueryResult.
#[allow(clippy::from_over_into)]
impl Into<RawQueryResult> for SendableQueryResult {
    fn into(mut self) -> RawQueryResult {
        RawQueryResult {
            result: self.result.take().unwrap(),
        }
    }
}

impl Drop for SendableQueryResult {
    fn drop(&mut self) {
        match self.result {
            None => (),
            // The SendableQueryResult was dropped before it was unwrapped.
            // This is likely to occur if you don't read the full results of a query.
            // This is because all the results will be sent, whether or not they are received.
            // This is simply how postgres works, it needs to finish reading one query before it goes on to the next query.
            Some(s) => unsafe { libpq::PQclear(s) },
        }
    }
}

/// The private struct containing the raw C pointer to the postgres connection.
/// The underlying unsafe libpq functions that take a *mut PGconn are accessed through impls on RawConnection.
/// We only want the underlying connection pointer to be used in a single thread.
/// From the docs: "As of version 17, libpq is always reentrant and thread-safe.
/// However, one restriction is that no two threads attempt to manipulate the same PGconn object at the same time.
/// In particular, you cannot issue concurrent commands from different threads through the same connection object.
/// (If you need to run concurrent commands, use multiple connections.)"
/// This will be enforced by rust as *mut is already !Send and !Sync, and also we are not impl copy nor clone.
pub(crate) struct RawConnection {
    conn: *mut libpq::PGconn,
}

impl Drop for RawConnection {
    fn drop(&mut self) {
        unsafe { libpq::PQfinish(self.conn) }
    }
}

// Custom methods of RawConnection.
impl RawConnection {
    /// Send a command to the database to be executed.
    /// Returns a SendableQueryResult wrapping the *mut PGResult.
    /// Will panic if the *mut PGResult is null, as this implies an out of memory error according to the docs.
    pub(crate) fn exec(&self, command: &str) -> SendableQueryResult {
        let command = std::ffi::CString::new(command)
            .expect("postgres queries should not contain internal nulls");
        let result: *mut libpq::PGresult = unsafe {
            libpq::PQexecParams(
                self.conn,
                command.into_raw(),
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
        assert!(
            !result.is_null(),
            "null pointer returned by libpq for a PGresult, suggesting lack of RAM"
        );
        SendableQueryResult {
            result: Some(result),
        }
    }
}

pub type ConnStatusType = libpq::ConnStatusType;

// Methods of RawConnection that are thing wrappers around methods on PQConn.
impl RawConnection {
    /// Makes a new connection to the database server.
    /// Blocks until the connection succeeds or fails. This can take minutes.
    /// Will panic if conninfo contains internal null bytes.
    /// Will panic if attempting to connect returns a null pointer.
    /// According to the docs this implies OOM:
    /// "Note that these functions will always return a non-null object pointer,
    /// unless perhaps there is too little memory even to allocate the PGconn object."
    pub(crate) fn PQconnectdb(conninfo: &str) -> Self {
        let conninfo = std::ffi::CString::new(conninfo)
            .expect("postgres connection strings should not contain internal nulls");
        let conn: *mut libpq::pg_conn = unsafe { libpq::PQconnectdb(conninfo.into_raw()) };
        assert!(
            !conn.is_null(),
            "null pointer returned by libpq when attempting to connect to postgres, suggesting lack of RAM"
        );
        RawConnection { conn }
    }

    pub(crate) fn PQstatus(&self) -> libpq::ConnStatusType {
        unsafe { libpq::PQstatus(self.conn) }
    }

    /// From the docs: "Returns the error message most recently generated by an operation on the connection."
    pub(crate) fn PQerrorMessage(&self) -> String {
        // SAFETY: The memory of the error message is valid for the lifetime of the connection, and we copy it immediately.
        let raw_error_message: &std::ffi::CStr =
            unsafe { std::ffi::CStr::from_ptr(libpq::PQerrorMessage(self.conn)) };
        let mut result: String = raw_error_message.to_string_lossy().into_owned();
        // Postgres error messages have a trailing newline, which I don't like.
        result.pop();
        result
    }

    pub(crate) fn PQsetNoticeReceiver(
        &self,
        func: unsafe extern "C" fn(arg: *mut ::std::ffi::c_void, res: *const libpq::PGresult),
        arg: *mut std::ffi::c_void,
    ) {
        unsafe { libpq::PQsetNoticeReceiver(self.conn, Some(func), arg) };
    }
}

pub unsafe extern "C" fn custom_notice_receiver(
    userdata: *mut std::ffi::c_void,
    pg_result: *const libpq::PGresult,
) {
    {
        // SAFETY: The memory of the error message is valid for the lifetime of the function call.
        // We copy it immediately to an owned value.
        // It's not documented, but we can see after calling the notice receiver function, that PGresult is cleared.
        // https://github.com/postgres/postgres/blob/REL_17_6/src/interfaces/libpq/fe-exec.c#L980
        // Therefore we should *NOT* call PGclear ourselves here!
        let message: &std::ffi::CStr =
            unsafe { std::ffi::CStr::from_ptr(libpq::PQresultErrorMessage(pg_result)) };
        let message: String = message.to_string_lossy().into_owned();

        // SAFETY: LOL idk prayge.
        // If there's a better way to do this let me know.
        #[allow(clippy::transmute_ptr_to_ref)]
        let s: &std::sync::mpsc::Sender<String> = unsafe { std::mem::transmute(userdata) };
        _ = s.send(message);
    }
}

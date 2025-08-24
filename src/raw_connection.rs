// We allow non_snake_case because it more convient to have the impls of RawConnection directly map to libpq functions.
#![allow(non_snake_case)]

use crate::libpq;

/// The private struct containing the raw C pointer to the postgres connection.
/// The underlying unsafe libpq functions are access through impls on RawConnection.
/// We only want the underlying connection pointer to be used in a single thread.
/// From the docs: "As of version 17, libpq is always reentrant and thread-safe.
/// However, one restriction is that no two threads attempt to manipulate the same PGconn object at the same time.
/// In particular, you cannot issue concurrent commands from different threads through the same connection object.
/// (If you need to run concurrent commands, use multiple connections.)"
/// This will be enforced by rust as *mut is already !Send and !Sync, and also we are not impl copy nor clone.
struct RawConnection {
    conn: *mut libpq::PGconn,
}

impl Drop for RawConnection {
    fn drop(&mut self) {
        unsafe { libpq::PQfinish(self.conn) }
    }
}

impl RawConnection {
    /// Makes a new connection to the database server.
    /// Blocks until the connection succeeds or fails. This can take minutes.
    /// Will panic if conninfo contains internal null bytes.
    /// Will panic if attempting to connect returns a null pointer.
    /// According to the docs this implies OOM:
    /// "Note that these functions will always return a non-null object pointer,
    /// unless perhaps there is too little memory even to allocate the PGconn object."
    pub(crate) fn PQconnectdb(&mut self, conninfo: &str) {
        let conninfo = std::ffi::CString::new(conninfo)
            .expect("postgres connection strings should not contain internal nulls");
        self.conn = unsafe { libpq::PQconnectdb(conninfo.into_raw()) };
        assert!(
            !self.conn.is_null(),
            "null pointer returned by libpq when attempting to connect to postgres"
        );
    }
}

use crate::connection::{BadConnection, Connection};
use crate::libpq;

impl Connection {
    /// Returns the server version, in the integer format used my libpq.
    pub fn server_version(self) -> (Result<Connection, BadConnection>, i64) {
        let version: i64 = (unsafe { libpq::PQserverVersion(self.raw()) }) as i64;
        if version > 0 {
            (Ok(self), version)
        } else {
            (Err(self.bad()), version)
        }
    }
}

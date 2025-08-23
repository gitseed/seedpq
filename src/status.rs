use crate::connection::{Connection, ConnectionError};
use crate::libpq;

impl Connection {
    /// Returns the server version, in the integer format used my libpq.
    pub fn server_version(&mut self) -> Result<i64, ConnectionError> {
        if !self.ok {
            Err(self.error())
        } else {
            let version: i64 = (unsafe { libpq::PQserverVersion(self.raw()) }) as i64;
            if version > 0 {
                self.ok = false;
                Err(self.error())
            } else {
                Ok(version)
            }
        }
    }
}

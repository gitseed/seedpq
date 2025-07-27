use crate::libpq;
pub struct Connection {
    conn: *mut libpq::PGconn
}

pub struct PendingConnection {
    conn: *mut libpq::PGconn
}

#[derive(Debug)]
pub struct ConnectionError {
    #[allow(dead_code)]
    message: String
}

impl Connection {
    pub fn new(connection_string: &str) -> PendingConnection {
        let raw_conninfo: *mut std::ffi::c_char = std::ffi::CString::new(connection_string)
            .expect("Postgres connection info should not contain internal null characters")
            .into_raw();
        let conn: *mut libpq::pg_conn = unsafe { libpq::PQconnectStart(raw_conninfo) };
        PendingConnection { conn }
    }

    pub fn server_version(&mut self) -> i64 {
        (unsafe { libpq::PQserverVersion(self.conn) }) as i64
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe { libpq::PQfinish(self.conn) }
    }
}

impl Future for PendingConnection {
    type Output = Result<Connection, ConnectionError>;

    // TODO: Probably important to use cx, probably things won't wake
    fn poll(self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let status: libpq::PostgresPollingStatusType = unsafe { libpq::PQconnectPoll(self.conn) };
        if status == libpq::PostgresPollingStatusType::PGRES_POLLING_OK {
            return std::task::Poll::Ready(Ok(Connection {
                conn: self.conn
            }))
        } else if status == libpq::PostgresPollingStatusType::PGRES_POLLING_FAILED {
            return std::task::Poll::Ready(Err(get_connection_error(self.conn)));
        } else {
            return std::task::Poll::Pending
        }
    }
}

fn get_connection_error(conn: *const libpq::PGconn) -> ConnectionError {
    let raw_error_message: &std::ffi::CStr =
        unsafe { std::ffi::CStr::from_ptr(libpq::PQerrorMessage(conn)) };
    ConnectionError {
        message: String::from(raw_error_message.to_str().unwrap()),
    }
}

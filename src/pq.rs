use crate::libpq;

#[derive(Debug)]
pub struct ConnectionError {
    #[allow(dead_code)]
    message: String,
}

pub fn connect(conninfo: &str) -> Result<*const libpq::PGconn, ConnectionError> {
    let raw_conninfo: *mut std::ffi::c_char = std::ffi::CString::new(conninfo)
        .expect("Postgres connection info should not contain internal null characters")
        .into_raw();
    let conn: *const libpq::PGconn = unsafe { libpq::PQconnectdb(raw_conninfo) };

    match status_ok(conn) {
        true => Ok(conn),
        false => Err(get_connection_error(conn)),
    }
}

fn get_connection_error(conn: *const libpq::PGconn) -> ConnectionError {
    let raw_error_message: &std::ffi::CStr =
        unsafe { std::ffi::CStr::from_ptr(libpq::PQerrorMessage(conn)) };
    ConnectionError {
        message: String::from(raw_error_message.to_str().unwrap()),
    }
}

fn status_ok(conn: *const libpq::PGconn) -> bool {
    let status: libpq::ConnStatusType = unsafe { libpq::PQstatus(conn) };
    if status == libpq::ConnStatusType::CONNECTION_OK {
        true
    } else {
        false
    }
}

pub fn server_version(conn: *const libpq::PGconn) -> i64 {
    (unsafe { libpq::PQserverVersion(conn) }) as i64
}

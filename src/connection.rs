use std::sync::mpsc;

use crate::libpq;

/// The private struct containing the raw C pointer to the postgres connection.
/// Has some implementations that apply to connections regardless of state.
/// Most notably drop, which will call PQFinish on the connection.
struct RawConnection {
    conn: *mut libpq::PGconn,
}

/// Represents an established connection, that is usable for doing things such as running queries.
/// Every implementation on Connection will return a Result (or future Result) with FailedConnection as the error type.
/// This is because a connection can go bad at any time and for any reason, as is the nature of computer networking.
pub struct Connection {
    conn: RawConnection,
}

/// A pending connection is not established or failed yet, but it impl Future, so it can be awaited to get a Result<Connection, BadConnection>
pub struct PendingConnection {
    conn: RawConnection,
    waker_send: std::sync::mpsc::Sender<Option<std::task::Waker>>,
}

/// A bad connection that can't be used for sending queries or similar.
pub struct BadConnection {
    conn: RawConnection,
}

#[derive(Debug)]
pub struct ConnectionError {
    #[allow(dead_code)]
    message: String,
}

impl Connection {
    #[allow(clippy::new_ret_no_self)] // I want new to be async by default. There will be a new_sync that will behave in the traditional way.
    pub fn new(connection_string: &str) -> PendingConnection {
        let raw_conninfo: *mut std::ffi::c_char = std::ffi::CString::new(connection_string)
            .expect("Postgres connection info should not contain internal null characters")
            .into_raw();
        let conn: *mut libpq::pg_conn = unsafe { libpq::PQconnectStart(raw_conninfo) };
        let (waker_send, waker_receive) = mpsc::channel::<Option<std::task::Waker>>();
        wakeup_when_socket_writable(conn, waker_receive);
        PendingConnection {
            conn: RawConnection { conn },
            waker_send,
        }
    }

    pub fn server_version(&mut self) -> i64 {
        (unsafe { libpq::PQserverVersion(self.conn.conn) }) as i64
    }
}

impl Future for PendingConnection {
    type Output = Result<Connection, ConnectionError>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let status: libpq::PostgresPollingStatusType =
            unsafe { libpq::PQconnectPoll(self.conn.conn) };
        if status == libpq::PostgresPollingStatusType::PGRES_POLLING_OK {
            self.waker_send.send(None).unwrap();
            std::task::Poll::Ready(Ok(Connection { conn: self.conn }))
        } else if status == libpq::PostgresPollingStatusType::PGRES_POLLING_FAILED {
            self.waker_send.send(None).unwrap();
            std::task::Poll::Ready(Err(get_connection_error(self.conn.conn)))
        } else {
            self.waker_send.send(Some(cx.waker().clone())).unwrap();
            std::task::Poll::Pending
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

fn wakeup_when_socket_writable(
    conn: *const libpq::PGconn,
    waker_receive: std::sync::mpsc::Receiver<Option<std::task::Waker>>,
) {
    let socket: std::ffi::c_int = unsafe { libpq::PQsocket(conn) };
    std::thread::spawn(move || {
        while let Some(s) = waker_receive.recv().unwrap() {
            // TODO: Add a timeout!!!
            unsafe { libpq::PQsocketPoll(socket, 0, 1, -1) };
            s.wake();
        }
    });
}

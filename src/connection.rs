use std::sync::mpsc;

use crate::libpq;

/// The private struct containing the raw C pointer to the postgres connection.
/// Has some implementations that apply to connections regardless of state.
/// Most notably drop, which will call PQFinish on the connection.
struct RawConnection {
    conn: *mut libpq::PGconn,
}

impl Drop for RawConnection {
    fn drop(&mut self) {
        unsafe { libpq::PQfinish(self.conn) }
    }
}

/// Represents a connection that's either established, or failed.
pub struct Connection {
    conn: RawConnection,
    pub(crate) ok: bool,
}

pub struct ConnectionError {
    message: String,
}

impl Connection {
    /// Returns the raw *mut of the given postgres connection.
    /// Accessing this through a getter allows us to be honest that this is mut.
    pub(crate) fn raw(&mut self) -> *mut libpq::PGconn {
        self.conn.conn
    }

    /// Returns the last error according to the postgres server.
    /// Also sets the connection as bad.
    pub fn error(&mut self) -> ConnectionError {
        self.ok = false;
        let raw_error_message: &std::ffi::CStr =
            unsafe { std::ffi::CStr::from_ptr(libpq::PQerrorMessage(self.raw())) };
        ConnectionError {
            message: String::from(raw_error_message.to_str().unwrap()),
        }
    }
}

/// Currently the same as display.
impl std::fmt::Debug for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

/// Display connection info.
impl std::fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "connection error: {}", self.message)
    }
}

impl std::error::Error for ConnectionError {}

/// A pending connection is not established or failed yet, that can be awaited to get a Connection.
/// The Connection may or may not be ok.
pub struct PendingConnection {
    // We need to use Option here. As far as the compiler knows Poll might be called after returning Ready.
    // Of course doing so will panic via unwrap() spam.
    // This means we can't transfer ownership directly, but have to do it via Option's take().
    // But this is effectively never None, unless you're performing some Future abuse.
    conn: Option<RawConnection>,
    waker_send: std::sync::mpsc::Sender<Option<std::task::Waker>>,
}

impl Future for PendingConnection {
    type Output = Connection;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let status: libpq::PostgresPollingStatusType =
            unsafe { libpq::PQconnectPoll(self.conn.as_ref().unwrap().conn) };
        if status == libpq::PostgresPollingStatusType::PGRES_POLLING_OK {
            self.waker_send.send(None).unwrap();
            std::task::Poll::Ready(Connection {
                conn: self.conn.take().unwrap(),
                ok: true,
            })
        } else if status == libpq::PostgresPollingStatusType::PGRES_POLLING_FAILED {
            self.waker_send.send(None).unwrap();
            std::task::Poll::Ready(Connection {
                conn: self.conn.take().unwrap(),
                ok: false,
            })
        } else {
            self.waker_send.send(Some(cx.waker().clone())).unwrap();
            std::task::Poll::Pending
        }
    }
}

/// Use the connection_string to attempt to establish a postgres connection.
/// Returns a PendingConnection that can be awaited to get a Result<Connection, FailedConnection>
/// If your connection_string includes a hostaddr parameter then this function should not block.
pub fn connect(connection_string: &str) -> PendingConnection {
    let raw_conninfo: *mut std::ffi::c_char = std::ffi::CString::new(connection_string)
        .expect("postgres connection info should not contain internal null characters")
        .into_raw();
    let conn: *mut libpq::pg_conn = unsafe { libpq::PQconnectStart(raw_conninfo) };
    let (waker_send, waker_receive) = mpsc::channel::<Option<std::task::Waker>>();
    wakeup_when_socket_writable(conn, waker_receive);
    PendingConnection {
        conn: Some(RawConnection { conn }),
        waker_send,
    }
}

/// Spawns a thread that blocks until it recieves a waker from supplied channel.
/// Once the spawned thread recieves the waiter, it blocks until the socket of the provided connection is ready for writing.
/// Once the socket is ready for writing, it will call wakeup on the Waker that it recieved earlier.
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

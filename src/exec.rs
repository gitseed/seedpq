use std::ptr::null;
use std::sync::mpsc;

use crate::connection::{Connection, ConnectionError};
use crate::query_result::{QueryError, QueryResult};

use crate::libpq;

use thiserror::Error;

/// This *is* exhuastive. You either get a error before you get the PGresult, or you get a dud PGresult.
#[derive(Error, Debug)]
pub enum ExecError {
    #[error(transparent)]
    ConnectionError(ConnectionError),
    #[error(transparent)]
    QueryError(QueryError),
}

/// A pending query that can be awaited to obtain a Result<QueryResult, QueryError>.
pub struct PendingQuery<'a> {
    conn: &'a mut Connection,
    waker_send: std::sync::mpsc::Sender<Option<std::task::Waker>>,
}

impl Future for PendingQuery<'_> {
    type Output = Result<QueryResult, ExecError>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        // From the docs: "PQisBusy will not itself attempt to read data from the server;
        // therefore PQconsumeInput must be invoked first, or the busy state will never end."
        if unsafe { libpq::PQconsumeInput(self.conn.raw()) } != 1 {
            self.waker_send.send(None).unwrap();
            // From the docs: "PQconsumeInput normally returns 1 indicating “no error”, but returns 0 if there was some kind of trouble."
            return std::task::Poll::Ready(Err(ExecError::ConnectionError(self.conn.error())));
        };

        // From the docs: "A 0 return indicates that PQgetResult can be called with assurance of not blocking."
        if unsafe { libpq::PQisBusy(self.conn.raw()) } == 0 {
            // Because we're using PQsendQueryParams there should only be one pg_result followed by a nullptr.
            let raw_result: *mut libpq::pg_result = unsafe { libpq::PQgetResult(self.conn.raw()) };
            assert!(!raw_result.is_null());
            let expecting_null: *mut libpq::pg_result =
                unsafe { libpq::PQgetResult(self.conn.raw()) };
            assert!(expecting_null.is_null());
            self.waker_send.send(None).unwrap();
            match result_ok(raw_result) {
                true => std::task::Poll::Ready(Ok(QueryResult { result: raw_result })),
                false => std::task::Poll::Ready(Err(ExecError::QueryError(QueryError {
                    result: raw_result,
                }))),
            }
        } else {
            self.waker_send.send(Some(cx.waker().clone())).unwrap();
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
            let (waker_send, waker_receive) = mpsc::channel::<Option<std::task::Waker>>();
            wakeup_when_socket_readable(self.raw(), waker_receive);
            Ok(PendingQuery {
                conn: self,
                waker_send,
            })
        }
    }
}

/// Spawns a thread that blocks until it recieves a waker from supplied channel.
/// Once the spawned thread recieves the waiter, it blocks until the socket of the provided connection is ready for writing.
/// Once the socket is ready for writing, it will call wakeup on the Waker that it recieved earlier.
fn wakeup_when_socket_readable(
    conn: *const libpq::PGconn,
    waker_receive: std::sync::mpsc::Receiver<Option<std::task::Waker>>,
) {
    let socket: std::ffi::c_int = unsafe { libpq::PQsocket(conn) };
    std::thread::spawn(move || {
        while let Some(s) = waker_receive.recv().unwrap() {
            // TODO: Add a timeout!!!
            unsafe { libpq::PQsocketPoll(socket, 1, 0, -1) };
            s.wake();
        }
    });
}

fn result_ok(raw_result: *mut libpq::PGresult) -> bool {
    let result_status: libpq::ExecStatusType = unsafe { libpq::PQresultStatus(raw_result) };
    matches!(
        result_status,
        libpq::ExecStatusType::PGRES_EMPTY_QUERY
            | libpq::ExecStatusType::PGRES_COMMAND_OK
            | libpq::ExecStatusType::PGRES_TUPLES_OK
    )
}

//! There is no "Connection" struct!
//! Instead "connect" returns a ConnectionSender, a ResultReceiver, InfoReceiver, and a NoticeReceiver.

use std::sync::mpsc::{Receiver, Sender, channel};

use crate::info;
use crate::info::{Info, InfoReceiver};
use crate::notice::NoticeReceiver;
use crate::query::{QueryReceiver, QueryResult};
use crate::query_error::QueryError;
use crate::query_raw::RawQueryResult;
use crate::raw_connection::RawConnection;
use crate::request::{PostgresRequest, RequestSender};

use crate::libpq::ConnStatusType;

/// Opens a postgres connecting using a connection string.
/// Connection strings are documented in 32.1.1. Connection Strings
/// Returns channels used for communcation with the postgres connection.
pub fn connect(
    connection_string: &str,
) -> (RequestSender, QueryReceiver, InfoReceiver, NoticeReceiver) {
    let (request_send, request_recv) = channel::<PostgresRequest>();
    let (query_send, query_recv) = channel::<RawQueryResult>();
    let (info_send, info_recv) = channel::<info::Info>();
    let (notice_send, notice_recv) = channel::<String>();
    let connection_string = String::from(connection_string);

    std::thread::spawn(move || {
        connection_event_loop(
            connection_string,
            request_recv,
            query_send,
            info_send,
            notice_send,
        );
    });
    (
        RequestSender { send: request_send },
        QueryReceiver { recv: query_recv },
        InfoReceiver { recv: info_recv },
        NoticeReceiver { recv: notice_recv },
    )
}

fn connection_event_loop(
    connection_string: String,
    request_recv: Receiver<PostgresRequest>,
    query_send: Sender<RawQueryResult>,
    info_send: Sender<info::Info>,
    notice_send: Sender<String>,
) {
    let conn: RawConnection = RawConnection::PQconnectdb(&connection_string);
    while let Ok(request) = request_recv.recv() {
        match request {
            PostgresRequest::Query(q) => {
                if conn.PQstatus() != ConnStatusType::CONNECTION_OK {
                    query_send.send(RawQueryResult {
                        query: q,
                        result: None,
                        connection_error_message: Some(String::from("Connection error")),
                    });
                } else {
                    todo!();
                }
            }
        }
    }
}

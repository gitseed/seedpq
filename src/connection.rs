//! There is no "Connection" struct!
//! Instead "connect" returns a ConnectionSender, a ResultReceiver, InfoReceiver, and a NoticeReceiver.

use std::sync::mpsc::{Receiver, Sender, channel};

use crate::info;
use crate::info::{Info, InfoReceiver};
use crate::notice::NoticeReceiver;
use crate::query::{QueryError, QueryReceiver, QueryResult};
use crate::request::{PostgresRequest, RequestSender};

/// Opens a postgres connecting using a connection string.
/// Connection strings are documented in 32.1.1. Connection Strings
/// Returns channels used for communcation with the postgres connection.
pub fn connect(
    connection_string: &str,
) -> (RequestSender, QueryReceiver, InfoReceiver, NoticeReceiver) {
    let (request_send, request_recv) = channel::<PostgresRequest>();
    let (query_send, query_recv) = channel::<Result<QueryResult, QueryError>>();
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
    result_send: Sender<Result<QueryResult, QueryError>>,
    info_send: Sender<info::Info>,
    notice_send: Sender<String>,
) {
    todo!()
}

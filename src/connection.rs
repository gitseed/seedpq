//! There is no "Connection" struct!
//! Instead "connect" returns a ConnectionSender, a ResultReceiver, InfoReceiver, and a NoticeReceiver.

use std::sync::mpsc::{Receiver, Sender, channel};

use crate::connection_error::ConnectionError;
use crate::connection_raw::{
    ConnStatusType, RawConnection, SendableQueryResult, custom_notice_receiver,
};
use crate::info;
use crate::info::InfoReceiver;
use crate::notice::NoticeReceiver;
use crate::query_recv::QueriesReceiver;
use crate::request::{PostgresRequest, RequestSender};

/// Opens a postgres connecting using a connection string.
/// Connection strings are documented in 32.1.1. Connection Strings
/// Returns channels used for communcation with the postgres connection.
pub fn connect(
    connection_string: &str,
) -> (RequestSender, QueriesReceiver, InfoReceiver, NoticeReceiver) {
    let (request_send, request_recv) = channel::<PostgresRequest>();
    let (query_send, query_recv) = channel::<(
        String,
        Result<Receiver<SendableQueryResult>, ConnectionError>,
    )>();
    let (info_send, info_recv) = channel::<info::Info>();
    let (notice_send, notice_recv) = channel::<String>();
    let connection_string = connection_string.to_owned();

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
        QueriesReceiver { recv: query_recv },
        InfoReceiver { recv: info_recv },
        NoticeReceiver(notice_recv),
    )
}

fn connection_event_loop(
    connection_string: String,
    request_recv: Receiver<PostgresRequest>,
    query_send: Sender<(
        String,
        Result<Receiver<SendableQueryResult>, ConnectionError>,
    )>,
    _info_send: Sender<info::Info>,
    mut notice_send: Sender<String>,
) {
    let conn: RawConnection = RawConnection::PQconnectdb(&connection_string);

    let notice_send_ptr: *mut Sender<String> = &mut notice_send;
    conn.PQsetNoticeReceiver(
        custom_notice_receiver,
        notice_send_ptr as *mut std::ffi::c_void,
    );

    while let Ok(request) = request_recv.recv() {
        match request {
            PostgresRequest::Query(query) => {
                let connection_status: ConnStatusType = conn.PQstatus();
                if connection_status == ConnStatusType::CONNECTION_OK {
                    let (s, r) = channel::<SendableQueryResult>();
                    let exec_result: SendableQueryResult = conn.exec(&query);
                    _ = query_send.send((query, Ok(r)));
                    _ = s.send(exec_result);
                } else {
                    _ = query_send.send((
                        query,
                        Err(ConnectionError::ConnectionBad {
                            status: connection_status,
                            msg: conn.PQerrorMessage(),
                        }),
                    ));
                }
            }
        }
    }
}

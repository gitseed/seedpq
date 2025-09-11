#![allow(warnings)]

use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use seedpq::libpq;
use tokio_postgres::{Error, NoTls};

pub fn get_insert_query() -> std::ffi::CString {
    const TIMES: usize = 10000;
    let mut values: String = String::new();
    for n in 0..TIMES {
        values.push_str("('User ");
        values.push_str(n.to_string().as_str());
        values.push_str("', NULL),");
    }
    // Remove the trailing comma.
    values.pop();

    std::ffi::CString::new(
        format!("insert into users (name, hair_color) VALUES {}", values).as_str(),
    )
    .unwrap()
}

pub struct Connection(*mut libpq::PGconn);
impl Drop for Connection {
    fn drop(&mut self) {
        unsafe { libpq::PQfinish(self.0) }
    }
}

unsafe extern "C" fn blackhole_notice_receiver(
    _: *mut std::ffi::c_void,
    _pg_result: *const libpq::pg_result,
) {
    {}
}

fn bench_trivial_libpq(b: &mut Bencher) {
    b.iter_batched(
        || {
            let insert_query = get_insert_query();

            unsafe {
                let c: Connection = Connection(libpq::PQconnectdb(c"postgres:///example".as_ptr()));
                // Avoids stdout spam while benchmarking...
                libpq::PQsetNoticeReceiver(
                    c.0,
                    Some(blackhole_notice_receiver),
                    std::ptr::null_mut(),
                );
                let result: *mut libpq::pg_result = libpq::PQexecParams(
                    c.0,
                    c"TRUNCATE TABLE comments CASCADE".as_ptr(),
                    0,
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null(),
                    1,
                );
                libpq::PQclear(result);
                let result: *mut libpq::pg_result = libpq::PQexecParams(
                    c.0,
                    c"TRUNCATE TABLE posts CASCADE".as_ptr(),
                    0,
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null(),
                    1,
                );
                libpq::PQclear(result);
                let result: *mut libpq::pg_result = libpq::PQexecParams(
                    c.0,
                    c"TRUNCATE TABLE users CASCADE".as_ptr(),
                    0,
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null(),
                    1,
                );
                libpq::PQclear(result);
                let result: *mut libpq::pg_result = libpq::PQexecParams(
                    c.0,
                    insert_query.as_ptr(),
                    0,
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null(),
                    1,
                );
                libpq::PQclear(result);
                c
            }
        },
        |c| unsafe {
            let result: *mut libpq::pg_result = libpq::PQexecParams(
                c.0,
                c"SELECT id, name, hair_color FROM users".as_ptr(),
                0,
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                1,
            );
            libpq::PQclear(result);
        },
        criterion::BatchSize::PerIteration,
    )
}

fn bench_trivial_tokio_postgres(b: &mut Bencher) {
    b.iter_batched(
        || {
            let rt: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
            let client: tokio_postgres::Client = rt.block_on(async {
                let (client, connection) =
                    tokio_postgres::connect("host=/tmp/ dbname=example", NoTls)
                        .await
                        .unwrap();

                tokio::spawn(async move { connection.await.unwrap() });
                client
                    .execute("TRUNCATE TABLE comments CASCADE", &[])
                    .await
                    .unwrap();
                client
                    .execute("TRUNCATE TABLE posts CASCADE", &[])
                    .await
                    .unwrap();
                client
                    .execute("TRUNCATE TABLE users CASCADE", &[])
                    .await
                    .unwrap();
                client
                    .execute(get_insert_query().to_str().unwrap(), &[])
                    .await
                    .unwrap();
                client
                    .execute("SELECT id, name, hair_color FROM users", &[])
                    .await
                    .unwrap();
                client
            });
            (rt, client)
        },
        |(rt, client)| {
            rt.block_on(async {
                client
                    .execute("SELECT id, name, hair_color FROM users", &[])
                    .await
                    .unwrap();
            })
        },
        criterion::BatchSize::PerIteration,
    );
}

fn bench_trivial_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_trivial_query");
    group.bench_function("trivial_libpq", bench_trivial_libpq);
    group.bench_function("trivial_tokio_postgres", bench_trivial_tokio_postgres);
}
criterion_group!(benches, bench_trivial_query);
criterion_main!(benches);

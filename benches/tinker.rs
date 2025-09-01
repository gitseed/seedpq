#![allow(warnings)]
use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use seedpq::libpq;
pub struct Connection(*mut libpq::PGconn);
impl Drop for Connection {
    fn drop(&mut self) {
        unsafe { libpq::PQfinish(self.0) }
        std::thread::sleep(std::time::Duration::from_micros(10000));
    }
}
pub fn bench_trivial_seed(b: &mut Bencher) {
    let c: Connection = Connection(unsafe { libpq::PQconnectdb(c"postgres:///example".as_ptr()) });
    unsafe {
        libpq::PQenterPipelineMode(c.0);
    }
    b.iter(|| {
        unsafe {
            libpq::PQsendQueryParams(
                c.0,
                c"SELECT id, name, hair_color FROM users".as_ptr(),
                0,
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                1,
            );
            libpq::PQsetChunkedRowsMode(c.0, 1000);
            libpq::PQpipelineSync(c.0);

            loop {
                let r: *mut libpq::pg_result = libpq::PQgetResult(c.0);
                if r.is_null() {
                    break;
                } else {
                    let status: libpq::ExecStatusType = libpq::PQresultStatus(r);
                    // println!("{}", std::ffi::CStr::from_ptr::<'static>(libpq::PQresStatus(status)).to_str().unwrap());
                    assert!(
                        status == libpq::ExecStatusType::PGRES_TUPLES_CHUNK
                            || status == libpq::ExecStatusType::PGRES_TUPLES_OK
                    );
                }
            }

            // PGRES_PIPELINE_SYNC, meaning all queries from the pipeline have been sent
            let r: *mut libpq::pg_result = libpq::PQgetResult(c.0);
            assert!(!r.is_null());
            assert!(libpq::PQresultStatus(r) == libpq::ExecStatusType::PGRES_PIPELINE_SYNC);
        };
    });
    std::hint::black_box(c);
}
fn bench_trivial_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_trivial_query");
    group.bench_function("tinker", bench_trivial_seed);
}
criterion_group!(benches, bench_trivial_query);
criterion_main!(benches);

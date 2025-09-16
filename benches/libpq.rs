use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use seedpq::libpq;

#[path = "common/common.rs"]
mod common;

pub struct Connection(*mut libpq::PGconn);
impl Drop for Connection {
    fn drop(&mut self) {
        unsafe { libpq::PQfinish(self.0) }
    }
}

fn bench_trivial_libpq(b: &mut Bencher) {
    b.iter_batched(
        || {
            common::setup_data();
            Connection(unsafe { libpq::PQconnectdb(c"postgres:///example".as_ptr()) })
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


fn bench_trivial_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_trivial_query");
    group.bench_function("trivial_libpq", bench_trivial_libpq);
}
criterion_group!(benches, bench_trivial_query);
criterion_main!(benches);

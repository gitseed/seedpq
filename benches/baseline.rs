use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use seedpq::libpq;
pub struct Connection(*mut libpq::PGconn);
impl Drop for Connection {
    fn drop(&mut self) {
        unsafe { libpq::PQfinish(self.0) }
    }
}
pub fn bench_trivial_seed(b: &mut Bencher) {
    let c: Connection = Connection(unsafe { libpq::PQconnectdb(c"postgres:///example".as_ptr()) });
    b.iter(|| {
        unsafe {
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
        };
    });
    std::hint::black_box(c);
}
fn bench_trivial_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_trivial_query");
    group.bench_function("baseline", bench_trivial_seed);
}
criterion_group!(benches, bench_trivial_query);
criterion_main!(benches);

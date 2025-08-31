#![allow(warnings)]

use criterion::{Bencher, Criterion, criterion_group, criterion_main};

use hybrid_array::Array;
use hybrid_array::typenum::U3;

use seedpq;
use seedpq::query::QueryReceiver;
use seedpq::query::QueryResult;
use seedpq::query_error::QueryDataError;

pub fn bench_trivial_seed(b: &mut Bencher) {
    const TIMES: usize = 10000;

    (|| -> Result<(), Box<dyn std::error::Error>> {
        let (s, r, _, _) = seedpq::connection::connect("postgres:///example");

        s.exec("TRUNCATE TABLE comments CASCADE");
        r.get::<seedpq::query::EmptyResult>()?;
        s.exec("TRUNCATE TABLE posts CASCADE");
        r.get::<seedpq::query::EmptyResult>()?;
        s.exec("TRUNCATE TABLE users CASCADE");
        r.get::<seedpq::query::EmptyResult>()?;

        Ok(())
    })()
    .unwrap();

    b.iter(|| todo!())
}

fn bench_trivial_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_trivial_query");
    group.bench_function("seed", bench_trivial_seed);
}

criterion_group!(benches, bench_trivial_query);
criterion_main!(benches);

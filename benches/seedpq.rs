use criterion::{Bencher, Criterion, criterion_group, criterion_main};

use seedpq::{EmptyResult, QueryReceiver, QueryResult};

#[path = "common/common.rs"]
mod common;

#[derive(Debug, QueryResult)]
#[allow(dead_code)]
struct User {
    id: i32,
    name: String,
    hair_color: Option<String>,
}

pub fn bench_trivial_seedpq(b: &mut Bencher) {
    b.iter_batched(
        || {
            common::setup_data();
            let (s, r, _, _) = seedpq::connect("postgres:///example");
            s.exec("select version()").unwrap();
            r.get::<EmptyResult>().unwrap();
            (s, r)
        },
        |(s, r)| {
            s.exec("SELECT id, name, hair_color FROM users").unwrap();
            let users: QueryReceiver<User> = r.get().unwrap();
            let result: Vec<User> = users.collect::<Result<Vec<User>, _>>().unwrap();
            assert_eq!(result.len(), 10000);
            result
        },
        criterion::BatchSize::PerIteration,
    )
}

fn bench_trivial_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_trivial_query");
    group.bench_function("seedpq", bench_trivial_seedpq);
}

criterion_group!(benches, bench_trivial_query);
criterion_main!(benches);

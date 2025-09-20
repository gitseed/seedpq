use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use postgres::{Client, NoTls};

#[path = "common/common.rs"]
mod common;

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    id: i32,
    name: String,
    hair_color: Option<String>,
}

#[allow(dead_code)]
fn bench_trivial_sfackler_postgres(b: &mut Bencher) {
    b.iter_batched(
        || {
            common::setup_data();
            let mut client = Client::connect("host=/tmp/ dbname=example", NoTls).unwrap();
            client.execute("select version()", &[]).unwrap();
            client
        },
        |mut client| {
            let rows = client
                .query("SELECT id, name, hair_color FROM users", &[])
                .unwrap();
            let result: Vec<User> = rows
                .iter()
                .map(|row| User {
                    id: row.get(0),
                    name: row.get(1),
                    hair_color: row.get(2),
                })
                .collect();
            assert_eq!(result.len(), 10000);
            result
        },
        criterion::BatchSize::PerIteration,
    );
}

fn bench_trivial_query(c: &mut Criterion) {
    #[allow(warnings)]
    let mut group = c.benchmark_group("bench_trivial_query");

    group.bench_function("sfackler_postgres", bench_trivial_sfackler_postgres);
}

criterion_group!(benches, bench_trivial_query);
criterion_main!(benches);

use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use tokio_postgres::NoTls;

#[path = "common/common.rs"]
mod common;

#[allow(dead_code)]
fn bench_trivial_tokio_postgres(b: &mut Bencher) {
    b.iter_batched(
        || {
            common::setup_data();
            let rt: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
            let client: tokio_postgres::Client = rt.block_on(async {
                let (client, connection) =
                    tokio_postgres::connect("host=/tmp/ dbname=example", NoTls)
                        .await
                        .unwrap();

                tokio::spawn(async move { connection.await.unwrap() });
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
    #[allow(warnings)]
    let mut group = c.benchmark_group("bench_trivial_query");

    // Tokio postgres is having its benchmarking permissions REVOKED!
    // This is due to being naughty, and causing errors in the postgres server logs!
    // group.bench_function("trivial_tokio_postgres", bench_trivial_tokio_postgres);
}

criterion_group!(benches, bench_trivial_query);
criterion_main!(benches);

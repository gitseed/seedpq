use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use wtx::database::client::postgres::PostgresExecutor;
use wtx::database::{Executor, Record, Records};
use wtx::rng::SeedableRng;

#[path = "common/common.rs"]
mod common;

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    id: i32,
    name: String,
    hair_color: Option<String>,
}

pub async fn executor_postgres(
    uri_str: &str,
) -> wtx::Result<
    wtx::database::client::postgres::PostgresExecutor<
        wtx::Error,
        wtx::database::client::postgres::ExecutorBuffer,
        tokio::net::UnixStream,
    >,
> {
    let uri = wtx::misc::Uri::new(uri_str);
    let mut rng = wtx::rng::ChaCha20::from_os().unwrap();
    let config = wtx::database::client::postgres::Config::from_uri(&uri).unwrap();
    let eb: wtx::database::client::postgres::ExecutorBuffer =
        wtx::database::client::postgres::ExecutorBuffer::new(usize::MAX, &mut rng);
    let stream: tokio::net::UnixStream = tokio::net::UnixStream::connect("/tmp/.s.PGSQL.5432")
        .await
        .unwrap();

    Ok(PostgresExecutor::connect(&config, eb, &mut rng, stream)
        .await
        .unwrap())
}

fn bench_trivial_wtx(b: &mut Bencher) {
    b.iter_batched(
        || {
            common::setup_data();
            let rt: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
            let e = rt.block_on(async {
                // I had to put a password even though unix socket auth doesn't use a password.
                // I have to put localhost even though it's not actually connecting over the network.
                executor_postgres("postgres://paul:notrealpassword@localhost/example")
                    .await
                    .unwrap()
            });
            (rt, e)
        },
        |(rt, mut e)| {
            rt.block_on(async {
                let data = e
                    .fetch_many_with_stmt("SELECT id, name, hair_color FROM users", (), |_| {
                        Ok::<_, wtx::Error>(())
                    })
                    .await
                    .unwrap();
                let result: Vec<User> = data
                    .iter()
                    .map(|record| User {
                        id: record.decode("id").unwrap(),
                        name: record.decode("name").unwrap(),
                        hair_color: record.decode_opt("hair_color").unwrap(),
                    })
                    .collect();
                result
            })
        },
        criterion::BatchSize::PerIteration,
    )
}

fn bench_trivial_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_trivial_query");
    group.bench_function("wtx", bench_trivial_wtx);
}
criterion_group!(benches, bench_trivial_query);
criterion_main!(benches);

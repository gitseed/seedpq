#![allow(warnings)]

use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use wtx::rng::SeedableRng;

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

pub async fn executor_postgres(
    uri_str: &str,
) -> wtx::Result<
    wtx::database::client::postgres::PostgresExecutor<
        wtx::Error,
        wtx::database::client::postgres::ExecutorBuffer,
        tokio::net::TcpStream,
    >,
> {
    let uri = wtx::misc::Uri::new(uri_str);
    let mut rng = wtx::rng::ChaCha20::from_os().unwrap();
    let config = wtx::database::client::postgres::Config::from_uri(&uri).unwrap();
    let eb = wtx::database::client::postgres::ExecutorBuffer::new(usize::MAX, &mut rng);
    let stream = tokio::net::TcpStream::connect(uri.hostname_with_implied_port())
        .await
        .unwrap();

    wtx::database::client::postgres::PostgresExecutor::connect(&config, eb, &mut rng, stream);
    todo!()
}

fn bench_trivial_wtx(b: &mut Bencher) {
    b.iter_batched(
        || {
            let rt: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                // wtx_instances::executor_postgres("postgres:///example").await?;
            });
            todo!();
            todo!()
        },
        |args| {
            todo!();
            todo!()
        },
        criterion::BatchSize::PerIteration,
    )
}

fn bench_trivial_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_trivial_query");
    group.bench_function("trivial_wtx", bench_trivial_wtx);
}
criterion_group!(benches, bench_trivial_query);
criterion_main!(benches);

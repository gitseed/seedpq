use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use seedpq::connection::{Connection, connect};
use tokio;

#[allow(dead_code)]
struct User<'a> {
    id: i32,
    name: &'a str,
    hair_color: Option<&'a str>,
}

impl<'a> From<[Option<&'a [u8]>; 3]> for User<'a> {
    fn from(item: [Option<&'a [u8]>; 3]) -> Self {
        User::<'a> {
            id: i32::from_be_bytes(item[0].unwrap().try_into().unwrap()),
            name: str::from_utf8(item[1].unwrap()).unwrap(),
            hair_color: match item[2] {
                None => None,
                Some(s) => Some(str::from_utf8(s).unwrap()),
            },
        }
    }
}

pub fn bench_trivial_seed(b: &mut Bencher) {
    const TIMES: usize = 10000;

    let runtime = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let mut c: Connection = runtime.block_on(async {
        let mut c: Connection = connect("postgres:///example").await;
        c.exec("TRUNCATE TABLE comments CASCADE")
            .unwrap()
            .await
            .unwrap();
        c.exec("TRUNCATE TABLE posts CASCADE")
            .unwrap()
            .await
            .unwrap();
        c.exec("TRUNCATE TABLE users CASCADE")
            .unwrap()
            .await
            .unwrap();
        for n in 0..TIMES {
            c.exec(
                format!(
                    "insert into users (name, hair_color) VALUES ('User {}', NULL)",
                    n.to_string()
                )
                .as_str(),
            )
            .unwrap()
            .await
            .unwrap();
        }
        c
    });

    b.iter(|| {
        runtime.block_on(async {
            let result: seedpq::query_result::QueryResult = c
                .exec("SELECT id, name, hair_color FROM users")
                .unwrap()
                .await
                .unwrap();
            result.fetch_all::<3, User>()
        })
    })
}

fn bench_trivial_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_trivial_query");
    group.bench_function("seed", bench_trivial_seed);
}

criterion_group!(benches, bench_trivial_query);
criterion_main!(benches);

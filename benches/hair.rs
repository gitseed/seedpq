use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use seedpq::connection::{Connection, connect};

use futures::executor;

#[allow(dead_code)]
struct User {
    id: i32,
    name: String,
    hair_color: Option<String>,
}

impl From<[Option<&[u8]>; 3]> for User {
    fn from(item: [Option<&[u8]>; 3]) -> Self {
        User {
            id: i32::from_be_bytes(item[0].unwrap().try_into().unwrap()),
            name: String::from_utf8_lossy(item[1].unwrap()).into_owned(),
            hair_color: match item[2] {
                None => None,
                Some(s) => Some(String::from_utf8_lossy(s).into_owned()),
            },
        }
    }
}

pub fn bench_trivial_seed(b: &mut Bencher) {
    const TIMES: usize = 10000;

    let mut c: Connection = executor::block_on(async {
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
        let mut values: String = String::new();

        for n in 0..TIMES {
            values.push_str("('User ");
            values.push_str(n.to_string().as_str());
            values.push_str("', NULL),");
        }
        // Remove the trailing comma.
        values.pop();
        c.exec(format!("insert into users (name, hair_color) VALUES {}", values).as_str())
            .unwrap()
            .await
            .unwrap();
        c
    });

    b.iter(|| {
        executor::block_on(async {
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

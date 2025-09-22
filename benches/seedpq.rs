use criterion::{Bencher, Criterion, criterion_group, criterion_main};

use hybrid_array::Array;
use hybrid_array::typenum::U3;

use seedpq::{EmptyResult, PostgresData, QueryReceiver, QueryResult, QueryResultError};

#[path = "common/common.rs"]
mod common;

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    id: i32,
    name: String,
    hair_color: Option<String>,
}

impl QueryResult<'_> for User {
    type Columns = U3;
    const COLUMN_NAMES: Array<&'static str, Self::Columns> = Array(["id", "name", "hair_color"]);
}

impl TryFrom<Array<PostgresData<'_>, U3>> for User {
    type Error = QueryResultError;

    fn try_from(data: Array<PostgresData, U3>) -> Result<Self, Self::Error> {
        let id: i32 = match data.0[0].try_into() {
            Ok(value) => Ok(value),
            Err(e) => Err(QueryResultError {
                e,
                t: std::any::type_name::<User>(),
                column: 0,
            }),
        }?;
        let name: String = match data.0[1].try_into() {
            Ok(value) => Ok(value),
            Err(e) => Err(QueryResultError {
                e,
                t: std::any::type_name::<User>(),
                column: 1,
            }),
        }?;
        let hair_color: Option<String> = match data.0[2].try_into() {
            Ok(value) => Ok(value),
            Err(e) => Err(QueryResultError {
                e,
                t: std::any::type_name::<User>(),
                column: 2,
            }),
        }?;
        Ok(User {
            id,
            name,
            hair_color,
        })
    }
}

pub fn bench_trivial_seedpq(b: &mut Bencher) {
    b.iter_batched(
        || {
            common::setup_data();
            let (s, r, _, _) = seedpq::connect("postgres:///example");
            s.exec("select version()", None).unwrap();
            r.get::<EmptyResult>().unwrap();
            (s, r)
        },
        |(s, r)| {
            s.exec("SELECT id, name, hair_color FROM users", None)
                .unwrap();
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

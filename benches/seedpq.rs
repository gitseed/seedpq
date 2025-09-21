use criterion::{Bencher, Criterion, criterion_group, criterion_main};

use hybrid_array::Array;
use hybrid_array::typenum::U3;

use seedpq::{EmptyResult, QueryDataError, QueryReceiver, QueryResult};

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

impl TryFrom<Array<Option<&[u8]>, U3>> for User {
    type Error = QueryDataError;

    fn try_from(data: Array<Option<&[u8]>, U3>) -> Result<Self, Self::Error> {
        let id: i32 = match data.0[0] {
            None => Err(QueryDataError::UnexpectedNullError {
                column: 0,
                t: std::any::type_name::<User>(),
            }),
            Some(data) => match <[u8; size_of::<i32>()]>::try_from(data) {
                Ok(arr) => Ok(i32::from_be_bytes(arr)),
                Err(e) => Err(QueryDataError::WrongSizeNumericError {
                    t: std::any::type_name::<i32>(),
                    e,
                    column: 0,
                    numsize: size_of::<i32>(),
                    slicesize: data.len(),
                }),
            },
        }?;
        let name: String = match data.0[1] {
            None => Err(QueryDataError::UnexpectedNullError {
                column: 1,
                t: std::any::type_name::<User>(),
            }),
            Some(data) => match str::from_utf8(data) {
                Ok(s) => Ok(s.to_owned()),
                Err(e) => Err(QueryDataError::Utf8Error {
                    e,
                    column: 1,
                    t: std::any::type_name::<User>(),
                }),
            },
        }?;
        let hair_color: Option<String> = match data.0[2] {
            None => Ok(None),
            Some(data) => match str::from_utf8(data) {
                Ok(s) => Ok(Some(s.to_owned())),
                Err(e) => Err(QueryDataError::Utf8Error {
                    e,
                    column: 2,
                    t: std::any::type_name::<User>(),
                }),
            },
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

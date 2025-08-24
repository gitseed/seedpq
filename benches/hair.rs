use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use seedpq::connection::{Connection, connect};
use tokio;

//                                                              Table "public.users"
//    Column   |       Type        | Collation | Nullable |              Default              | Storage  | Compression | Stats target | Description 
// ------------+-------------------+-----------+----------+-----------------------------------+----------+-------------+--------------+-------------
//  id         | integer           |           | not null | nextval('users_id_seq'::regclass) | plain    |             |              | 
//  name       | character varying |           | not null |                                   | extended |             |              | 
//  hair_color | character varying |           |          |                                   | extended |             |              | 
#[allow(dead_code)]
struct Users<'a> {
    id: i32,
    name: &'a str,
    hair_color: Option<&'a str>
}

impl <'a>From<[Option<&'a [u8]>; 3]> for Users<'a> {
    fn from(item: [Option<&'a [u8]>; 3]) -> Self {
        Users::<'a> {
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
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let mut c: Connection = runtime.block_on(async { connect("postgres:///example").await });

    b.iter(|| {
        runtime.block_on(async {
            let result: seedpq::query_result::QueryResult = c.exec("SELECT id, name, hair_color FROM users;")
                .unwrap()
                .await;
            let _output: Vec<Users> = result.fetch_all::<3, Users>();
        })
    })
}

fn bench_trivial_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_trivial_query");
    group.bench_function("seed", bench_trivial_seed);
}

criterion_group!(benches, bench_trivial_query);
criterion_main!(benches);

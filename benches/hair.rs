use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use seedpq::connection::{Connection, connect_sync};

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

    let c: Connection = executor::block_on(async {
        let mut c: Connection = connect_sync("postgres:///example");
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
        let query: &'static str = "SELECT id, name, hair_color FROM users";
        let ffi_query: std::ffi::CString = std::ffi::CString::new(query).unwrap();
        unsafe {
            seedpq::libpq::PQexecParams(
                c.conn.conn,
                ffi_query.as_ptr(),
                0,
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                // Specify zero to obtain results in text format, or one to obtain results in binary format.
                // If you specify text format then numbers wil be sent in text form which is dumb.
                1,
            )
        };
    })
}

fn bench_trivial_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_trivial_query");
    group.bench_function("seed", bench_trivial_seed);
}

criterion_group!(benches, bench_trivial_query);
criterion_main!(benches);

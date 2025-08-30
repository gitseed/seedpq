use seedpq::connection::{Connection, connect};

use futures::executor;

fn main() {
    executor::block_on(async_main()).unwrap();
}

#[allow(dead_code)]
#[derive(Debug)]
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

const TIMES: usize = 10;

async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut c: Connection = connect("postgres:///example").await;
    c.exec("TRUNCATE TABLE comments CASCADE")?.await?;
    c.exec("TRUNCATE TABLE posts CASCADE")?.await?;
    c.exec("TRUNCATE TABLE users CASCADE")?.await?;
    for n in 0..TIMES {
        c.exec(
            format!(
                "insert into users (name, hair_color) VALUES ('User {}', NULL)",
                n.to_string()
            )
            .as_str(),
        )?
        .await?;
    }
    let result: seedpq::query_result::QueryResult =
        c.exec("SELECT id, name, hair_color FROM users;")?.await?;
    let user: User = result.fetch_one::<3, User>();
    println!("{:?}", user);
    Ok(())
}

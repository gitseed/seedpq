use seedpq::connection::{Connection, connect};

#[allow(dead_code)]
#[derive(Debug)]
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

const TIMES: usize = 10;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

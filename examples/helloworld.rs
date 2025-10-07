use seedpq::QueryResult;

#[derive(QueryResult, Debug)]
#[allow(dead_code)]
struct User {
    id: i32,
    name: String,
    hair_color: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let (db_send, db_recv, _, _) = seedpq::connect("postgres:///example");
    db_send.exec("select * from users limit 5")?;
    let users: Vec<User> = db_recv.get()?.all()?;
    dbg!(users);
    db_send.exec("select version()")?;
    let version: String = db_recv.get()?.one()?;
    dbg!(version);
    Ok(())
}

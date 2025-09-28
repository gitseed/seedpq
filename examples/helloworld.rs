use seedpq::{QueryReceiver, QueryResult};

#[derive(QueryResult, Debug)]
#[allow(dead_code)]
struct User {
    id: i32,
    name: String,
    hair_color: Option<String>,
}

fn _main() -> Result<(), Box<dyn std::error::Error>> {
    let (s, r, _, _) = seedpq::connect("postgres:///example");
    s.exec("select * from users limit 5", None)?;
    let users: QueryReceiver<User> = r.get()?;
    let users: Vec<User> = users.all()?;
    dbg!(users);
    s.exec("select version()", None).unwrap();
    let version: QueryReceiver<String> = r.get().unwrap();
    let version: String = version.one().unwrap();
    dbg!(version);
    Ok(())
}

fn main() {
    match _main() {
        Ok(()) => (),
        Err(e) => println!("{}", e),
    }
}

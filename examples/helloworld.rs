use seedpq::QueryResult;

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
    let users: Vec<User> = r.get()?.all()?;
    dbg!(users);
    s.exec("select version()", None)?;
    let version: String = r.get()?.one()?;
    dbg!(version);
    Ok(())
}

fn main() {
    match _main() {
        Ok(()) => (),
        Err(e) => println!("{}", e),
    }
}

use seedpq::{QueryReceiver, QueryResult};

#[derive(QueryResult, Debug)]
#[allow(dead_code)]
struct User {
    id: i32,
    name: String,
    hair_color: Option<String>,
}

fn main() {
    let (s, r, _, _) = seedpq::connect("postgres:///example");
    s.exec("select * from users limit 5", None).unwrap();
    let users: QueryReceiver<User> = r.get().unwrap();
    let users: Vec<User> = users.collect::<Result<Vec<User>, _>>().unwrap();
    dbg!(users);
    s.exec("select version()", None).unwrap();
    let version: QueryReceiver<String> = r.get().unwrap();
    let version: Vec<String> = version.collect::<Result<Vec<String>, _>>().unwrap();
    dbg!(version);
}

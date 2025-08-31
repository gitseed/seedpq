use hybrid_array::Array;
use hybrid_array::typenum::U3;
use seedpq;
use seedpq::query::QueryReceiver;
use seedpq::query::QueryResult;
use seedpq::query_error::QueryDataError;

fn main() {
    match _main() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    hair_color: Option<String>,
}

impl QueryResult<'_> for User {
    type Columns = U3;
}

impl TryFrom<Array<Option<&[u8]>, U3>> for User {
    type Error = QueryDataError;

    fn try_from(data: Array<Option<&[u8]>, U3>) -> Result<Self, Self::Error> {
        todo!()
    }
}

fn _main() -> Result<(), Box<dyn std::error::Error>> {
    let (s, r, _, _) = seedpq::connection::connect("postgres:///example");

    s.exec("select * from users limit 10");
    let mut users: QueryReceiver<User> = r.get::<User>()?;

    let users: Vec<User> = users.collect::<Result<_, _>>()?;

    Ok(())
}

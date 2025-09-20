use hybrid_array::Array;
use hybrid_array::typenum::U1;
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
struct PostgresVersionInfo {
    info: String,
}

impl TryFrom<Array<Option<&[u8]>, U1>> for PostgresVersionInfo {
    type Error = QueryDataError;

    fn try_from(data: Array<Option<&[u8]>, U1>) -> Result<Self, Self::Error> {
        match data.0[0] {
            None => Err(QueryDataError::UnexpectedNullError {
                column: 0,
                t: std::any::type_name::<PostgresVersionInfo>(),
            }),
            Some(data) => match str::from_utf8(data) {
                Ok(s) => Ok(PostgresVersionInfo { info: s.to_owned() }),
                Err(e) => Err(QueryDataError::Utf8Error {
                    e,
                    column: 0,
                    t: std::any::type_name::<PostgresVersionInfo>(),
                }),
            },
        }
    }
}

impl QueryResult<'_> for PostgresVersionInfo {
    type Columns = U1;
    const COLUMN_NAMES: Array<&'static str, Self::Columns> = Array(["version"]);
}

fn _main() -> Result<(), Box<dyn std::error::Error>> {
    let (s, r, _, _) = seedpq::connection::connect("postgres:///example");

    s.exec("SELECT version()")?;
    let mut version: QueryReceiver<PostgresVersionInfo> = r.get::<PostgresVersionInfo>()?;
    println!("{}", version.next().unwrap().unwrap().info);
    Ok(())
}

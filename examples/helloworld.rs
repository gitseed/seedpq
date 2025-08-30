use seedpq;

use hybrid_array::typenum::U1;

use hybrid_array::Array;

fn main() {
    match _main() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}

#[derive(Debug)]
struct PostgresVersionInfo<'a> {
    info: &'a str,
}

impl<'a> seedpq::query::QueryResult<'a> for PostgresVersionInfo<'a> {
    type Columns = U1;
}

impl<'a> From<Array<Option<&'a [u8]>, U1>> for PostgresVersionInfo<'a> {
    fn from(data: Array<Option<&'a [u8]>, U1>) -> Self {
        PostgresVersionInfo {
            info: str::from_utf8(data.0[0].unwrap()).unwrap(),
        }
    }
}

fn _main() -> Result<(), Box<dyn std::error::Error>> {
    let (s, r, _, _) = seedpq::connection::connect("postgres:///example");

    s.exec("SELECT version()");
    let version: seedpq::query::QueryReceiver<PostgresVersionInfo> =
        r.get::<PostgresVersionInfo>()?;
    dbg!(version.fetch_one());
    Ok(())
}

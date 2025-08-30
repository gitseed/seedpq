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
struct PostgresVersionInfo {
    info: String,
}

impl From<Array<Option<&[u8]>, U1>> for PostgresVersionInfo {
    fn from(data: Array<Option<&[u8]>, U1>) -> Self {
        PostgresVersionInfo {
            info: String::from_utf8_lossy(data.0[0].unwrap()).into_owned(),
        }
    }
}

// impl<'a> seedpq::query::QueryResult<'a> for PostgresVersionInfo {
//     type Columns = U1;
// }

fn _main() -> Result<(), Box<dyn std::error::Error>> {
    let (s, r, _, _) = seedpq::connection::connect("postgres:///example");

    // s.exec("SELECT version()");
    // let mut version: seedpq::query::QueryReceiver<PostgresVersionInfo> =
    //     r.get::<PostgresVersionInfo>()?;
    // dbg!(version.next().unwrap().info);
    Ok(())
}

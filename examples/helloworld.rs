use hybrid_array::Array;
use hybrid_array::typenum::U1;
use seedpq;
use seedpq::HelloMacro;

fn main() {
    match _main() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}

pub trait HelloMacro {
    fn hello_macro();
}

#[derive(Debug, HelloMacro)]
struct PostgresVersionInfo {
    info: String,
}

impl TryFrom<Array<Option<&[u8]>, U1>> for PostgresVersionInfo {
    type Error = seedpq::QueryDataError;

    fn try_from(data: Array<Option<&[u8]>, U1>) -> Result<Self, Self::Error> {
        match data.0[0] {
            None => Err(seedpq::QueryDataError::UnexpectedNullError {
                column: 0,
                t: std::any::type_name::<PostgresVersionInfo>(),
            }),
            Some(data) => match str::from_utf8(data) {
                Ok(s) => Ok(PostgresVersionInfo { info: s.to_owned() }),
                Err(e) => Err(seedpq::QueryDataError::Utf8Error {
                    e,
                    column: 0,
                    t: std::any::type_name::<PostgresVersionInfo>(),
                }),
            },
        }
    }
}

impl seedpq::QueryResult<'_> for PostgresVersionInfo {
    type Columns = U1;
    const COLUMN_NAMES: Array<&'static str, Self::Columns> = Array(["version"]);
}



fn _main() -> Result<(), Box<dyn std::error::Error>> {
    PostgresVersionInfo::hello_macro();

    let (s, r, _, _) = seedpq::connect("postgres:///example");

    s.exec("SELECT version()", None)?;
    let mut version: seedpq::QueryReceiver<PostgresVersionInfo> = r.get::<PostgresVersionInfo>()?;
    println!("{}", version.next().unwrap().unwrap().info);
    Ok(())
}

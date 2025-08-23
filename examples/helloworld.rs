use seedpq::connection::{Connection, connect};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut c: Connection = connect("postgres:///gitseed").await;
    let version = c.server_version()?;
    println!("{}", version);
    Ok(())
}

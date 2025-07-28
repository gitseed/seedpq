use seedpq::connection::{BadConnection, GoodConnection, connect};

#[tokio::main]
async fn main() -> Result<(), BadConnection> {
    let c: Result<GoodConnection, BadConnection> = connect("postgres:///gitseed").await;
    let (c, version) = c?.server_version();
    println!("{}", version);
    let (_, version) = c?.server_version();
    println!("{}", version);
    Ok(())
}

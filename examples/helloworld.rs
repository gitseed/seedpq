use seedpq::connection::{BadConnection, GoodConnection, connect};

#[tokio::main]
async fn main() {
    let c: Result<GoodConnection, BadConnection> = connect("postgres:///gitseed").await;
    let (c, version) = c.unwrap().server_version();
    println!("{}", version);
    let (_, version) = c.unwrap().server_version();
    println!("{}", version);
}

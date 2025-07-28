use seedpq::connection::{BadConnection, Connection, connect};

#[tokio::main]
async fn main() {
    let c: Result<Connection, BadConnection> = connect("postgres:///gitseed").await;
    let (c, version) = c.unwrap().server_version();
    println!("{}", version);
    let (_, version) = c.unwrap().server_version();
    println!("{}", version);
}

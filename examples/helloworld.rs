use seedpq::connection::{Connection, connect};

#[tokio::main]
async fn main() {
    let mut c: Connection = connect("postgres:///doesnotexist").await.unwrap();
    println!("{}", c.server_version());
}

use seedpq;

#[tokio::main]
async fn main() {
    let test_connection_string: &'static str = "postgres:///gitseed";
    let pending: seedpq::connection::PendingConnection = seedpq::connection::Connection::new(test_connection_string);
    let mut c = pending.await.unwrap();
    println!("{}", c.server_version());
}

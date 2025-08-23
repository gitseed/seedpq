use seedpq::connection::{Connection, connect};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut c: Connection = connect("postgres:///example").await;
    let result = c.exec("SELECT version();")?.await;
    println!("{result}");
    Ok(())
}

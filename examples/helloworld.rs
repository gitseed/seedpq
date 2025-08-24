use seedpq::connection::{Connection, connect};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut c: Connection = connect("postgres:///example").await;
    let result = c.exec("SELECT version();")?.await;
    let cell = result.fetch_cell(0, 0);
    let message = std::str::from_utf8(cell)?;
    println!("{}", message);
    Ok(())
}

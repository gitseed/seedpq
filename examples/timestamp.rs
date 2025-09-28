
fn _main() -> Result<(), Box<dyn std::error::Error>> {
    let (db_send, db_recv, _, _) = seedpq::connect("postgres:///gitseed");
    db_send.exec("truncate table timestamps cascade")?;
    db_send.exec("insert into timestamps (ts) values ('infinity');")?;
    db_send.exec("insert into timestamps (ts) values ('-infinity');")?;
    db_recv.get()?.none()?;
    db_recv.get()?.none()?;
    db_recv.get()?.none()?;
    db_send.exec("select * from timestamps")?;
    let mut result: seedpq::QueryReceiver<i64> = db_recv.get()?;
    let pg_inf: i64 = result.next().unwrap()?;
    assert_eq!(pg_inf, i64::MAX);
    let pg_neginf: i64 = result.next().unwrap()?;
    assert_eq!(pg_neginf, i64::MIN);
    Ok(())
}

fn main() {
    match _main() {
        Ok(()) => (),
        Err(e) => println!("{}", e),
    }
}

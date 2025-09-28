fn _main() -> Result<(), Box<dyn std::error::Error>> {
    let (db_send, db_recv, _, _) = seedpq::connect("postgres:///gitseed");
    db_send.exec("truncate table timestamp cascade")?;
    db_send.exec("insert into timestamp(epoch)")?;
    db_send.exec("insert into timestamp(infinity)")?;
    db_send.exec("insert into timestamp(-infinity)")?;
    db_send.exec("insert into timestamp(now)")?;
    // db_send.exec("select * from timestamp")?;
    db_recv.get()?.none()?;
    db_recv.get()?.none()?;
    db_recv.get()?.none()?;
    db_recv.get()?.none()?;
    db_recv.get()?.none()?;
    Ok(())
}

fn main() {
    match _main() {
        Ok(()) => (),
        Err(e) => println!("{}", e),
    }
}

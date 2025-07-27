use seedpq;

fn main() {
    let test_connection_string: &'static str = "postgres:///gitseed";
    let c: *const seedpq::libpq::PGconn = seedpq::pq::connect(test_connection_string).unwrap();
    let version = seedpq::pq::server_version(c);
    print!("{version}");
}

use seedpq;

fn main() {
    _main().unwrap()
}

fn _main() -> Result<(), Box<dyn std::error::Error>> {
    let (s, r, _, _) = seedpq::connection::connect("postgres:///example");

    s.exec("SELECT version()");
    r.get::<3>()?;
    Ok(())
}

fn send_ten_thousand_queries(s: seedpq::request::RequestSender) {
    s.exec("SELECT version()");
}

fn receive_ten_thousand_results(_r: seedpq::query_recv::QueriesReceiver) {}

use seedpq;

fn main() {
    _main().unwrap()
}

fn _main() -> Result<(), Box<dyn std::error::Error>> {
    let (s, r, _, _) = seedpq::connection::connect("postgres:///example");

    let s_handle: std::thread::JoinHandle<()> =
        std::thread::spawn(move || send_ten_thousand_queries(s));
    let r_handle: std::thread::JoinHandle<()> =
        std::thread::spawn(move || receive_ten_thousand_results(r));

    s_handle.join().unwrap();
    r_handle.join().unwrap();
    Ok(())
}

fn send_ten_thousand_queries(_s: seedpq::request::RequestSender) {
    todo!()
}

fn receive_ten_thousand_results(_r: seedpq::query::QueryReceiver) {
    todo!()
}

use seedpq::EmptyResult;

#[allow(dead_code)]
pub fn get_insert_query() -> String {
    const TIMES: usize = 10000;
    let mut values: String = String::new();
    for n in 0..TIMES {
        values.push_str("('User ");
        values.push_str(n.to_string().as_str());
        values.push_str("', NULL),");
    }
    // Remove the trailing comma.
    values.pop();

    format!("insert into users (name, hair_color) VALUES {}", values)
}

#[allow(dead_code)]
pub fn setup_data() {
    let (s, r, _, _) = seedpq::connect("postgres:///example");
    s.exec("TRUNCATE TABLE comments CASCADE", None).unwrap();
    s.exec("TRUNCATE TABLE posts CASCADE", None).unwrap();
    s.exec("TRUNCATE TABLE users CASCADE", None).unwrap();
    s.exec(&get_insert_query(), None).unwrap();
    assert!(r.get::<EmptyResult>().unwrap().next().is_none());
    assert!(r.get::<EmptyResult>().unwrap().next().is_none());
    assert!(r.get::<EmptyResult>().unwrap().next().is_none());
    assert!(r.get::<EmptyResult>().unwrap().next().is_none());
}

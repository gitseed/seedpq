use seedpq::libpq;

pub fn get_insert_query() -> std::ffi::CString {
    const TIMES: usize = 10000;
    let mut values: String = String::new();
    for n in 0..TIMES {
        values.push_str("('User ");
        values.push_str(n.to_string().as_str());
        values.push_str("', NULL),");
    }
    // Remove the trailing comma.
    values.pop();

    std::ffi::CString::new(
        format!("insert into users (name, hair_color) VALUES {}", values).as_str(),
    )
    .unwrap()
}

#[allow(dead_code)]
unsafe extern "C" fn blackhole_notice_receiver(
    _: *mut std::ffi::c_void,
    _pg_result: *const libpq::pg_result,
) {
    {}
}

#[allow(dead_code)]
pub fn setup_data() {
    unsafe {
        let c: *mut libpq::pg_conn = libpq::PQconnectdb(c"postgres:///example".as_ptr());
        // Avoids stdout spam while benchmarking...
        libpq::PQsetNoticeReceiver(c, Some(blackhole_notice_receiver), std::ptr::null_mut());
        let result: *mut libpq::pg_result = libpq::PQexecParams(
            c,
            c"TRUNCATE TABLE comments CASCADE".as_ptr(),
            0,
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
            1,
        );
        libpq::PQclear(result);
        let result: *mut libpq::pg_result = libpq::PQexecParams(
            c,
            c"TRUNCATE TABLE posts CASCADE".as_ptr(),
            0,
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
            1,
        );
        libpq::PQclear(result);
        let result: *mut libpq::pg_result = libpq::PQexecParams(
            c,
            c"TRUNCATE TABLE users CASCADE".as_ptr(),
            0,
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
            1,
        );
        libpq::PQclear(result);
        let result: *mut libpq::pg_result = libpq::PQexecParams(
            c,
            get_insert_query().as_ptr(),
            0,
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
            1,
        );
        libpq::PQclear(result);
        libpq::PQfinish(c);
    }
}

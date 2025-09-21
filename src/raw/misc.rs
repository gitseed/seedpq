use super::libpq;

/// Gets a static lifetime str from an ExecStatusType.
/// While this can theoretically panic, in practice it won't unless your libpq library is corrupted or similar.
pub(crate) fn PQresStatus(status: libpq::ExecStatusType) -> &'static str {
    unsafe {
        let raw: *mut std::ffi::c_char = libpq::PQresStatus(status);
        std::ffi::CStr::from_ptr::<'static>(raw).to_str().unwrap()
    }
}

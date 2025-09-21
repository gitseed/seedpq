// We allow non_snake_case because it more convenient to have the impls of RawQueryResult directly map to libpq functions.
#![allow(non_snake_case)]

use crate::libpq;

/// A private struct containing the raw C pointer to a PGresult.
/// Dropping this will call
#[derive(Debug)]
pub(crate) struct RawQueryResult(*mut libpq::PGresult);

impl Drop for RawQueryResult {
    fn drop(&mut self) {
        unsafe { libpq::PQclear(self.0) }
    }
}

// SAFETY: You can't clone or copy a RawQueryResult, and you can only use the pointer through the impls.
// Therefore its probably fine.
// Also libpq says query results are quite thread safe anyway, except in some edge cases.
unsafe impl Send for RawQueryResult {}

// Custom methods on RawQueryResult.
impl RawQueryResult {
    pub(crate) fn new(raw_ptr: *mut libpq::PGresult) -> Self {
        RawQueryResult(raw_ptr)
    }

    pub(crate) fn fetch_cell(&self, row: usize, column: usize) -> Option<&[u8]> {
        unsafe {
            // Pointer postgres gives is signed bytes, but rust wants raw data to be unsigned bytes. It's the same bytes though.
            let data: *mut u8 = libpq::PQgetvalue(self.0, row as i32, column as i32) as *mut u8;
            match self.PQgetisnull(row, column) {
                true => None,
                false => Some(std::slice::from_raw_parts(
                    data,
                    self.PQgetlength(row, column),
                )),
            }
        }
    }
}

pub type ExecStatusType = libpq::ExecStatusType;

// Methods of RawQueryResult that are thin wrappers around methods on PQConn.
impl RawQueryResult {
    pub(crate) fn PQgetisnull(&self, row: usize, column: usize) -> bool {
        (unsafe { libpq::PQgetisnull(self.0, row as i32, column as i32) } == 1)
    }

    pub(crate) fn PQgetlength(&self, row: usize, column: usize) -> usize {
        (unsafe { libpq::PQgetlength(self.0, row as i32, column as i32) } as usize)
    }

    pub(crate) fn PQntuples(&self) -> usize {
        (unsafe { libpq::PQntuples(self.0) } as usize)
    }

    pub(crate) fn PQnfields(&self) -> usize {
        (unsafe { libpq::PQnfields(self.0) } as usize)
    }

    pub(crate) fn PQresultStatus(&self) -> ExecStatusType {
        unsafe { libpq::PQresultStatus(self.0) }
    }

    pub(crate) fn PQfname(&self, column_number: usize) -> String {
        unsafe {
            let raw_ptr: *mut i8 = libpq::PQfname(self.0, column_number as i32);
            std::ffi::CStr::from_ptr(raw_ptr)
        }
        .to_string_lossy()
        .into_owned()
    }
}

/// Gets a static lifetime str from an ExecStatusType.
/// While this can theoretically panic, in practice it won't unless your libpq library is corrupted or similar.
pub(crate) fn PQresStatus(status: ExecStatusType) -> &'static str {
    unsafe {
        let raw: *mut std::ffi::c_char = libpq::PQresStatus(status);
        std::ffi::CStr::from_ptr::<'static>(raw).to_str().unwrap()
    }
}

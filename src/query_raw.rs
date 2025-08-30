use crate::libpq;

/// A private struct containing the raw C pointer to a PGresult.
/// Dropping this will call
#[derive(Debug)]
pub(crate) struct RawQueryResult {
    /// Private! We only want SendableQueryResult being constructed by wrapper functions that would return *mut PGresult
    pub(crate) result: *mut libpq::PGresult,
}

impl Drop for RawQueryResult {
    fn drop(&mut self) {
        unsafe { libpq::PQclear(self.result) }
    }
}

impl RawQueryResult {
    pub(crate) fn fetch_cell(&self, row: usize, column: usize) -> Option<&[u8]> {
        let row: i32 = row as i32;
        let column: i32 = column as i32;
        unsafe {
            // Pointer postgres gives is signed bytes, but rust wants raw data to be unsigned bytes. It's the same bytes though.
            let data: *mut u8 = libpq::PQgetvalue(self.result, row, column) as *mut u8;
            if libpq::PQgetisnull(self.result, row, column) == 1 {
                None
            } else {
                let len: usize = libpq::PQgetlength(self.result, row, column) as usize;
                Some(std::slice::from_raw_parts(data, len))
            }
        }
    }
}

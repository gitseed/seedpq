use crate::libpq;

#[derive(Debug)]
pub struct QueryResult {
    pub(crate) result: *mut libpq::PGresult,
}

impl Drop for QueryResult {
    fn drop(&mut self) {
        unsafe { libpq::PQclear(self.result) }
    }
}

impl QueryResult {
    pub fn fetch_cell(&self, row: i32, column: i32) -> &[u8] {
        unsafe {
            let len: usize = libpq::PQgetlength(self.result, row, column) as usize;
            // Pointer postgres gives is signed bytes, but rust wants raw data to be unsigned bytes. It's the same bytes though.
            let data: *mut u8 = libpq::PQgetvalue(self.result, row, column) as *mut u8;
            std::slice::from_raw_parts(data, len)
        }
    }
}

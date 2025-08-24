use crate::libpq;

pub struct QueryError {
    pub(crate) result: *mut libpq::PGresult,
}

impl Drop for QueryError {
    fn drop(&mut self) {
        unsafe { libpq::PQclear(self.result) }
    }
}

impl std::fmt::Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let raw_message: &std::ffi::CStr =
            unsafe { std::ffi::CStr::from_ptr(libpq::PQresultErrorMessage(self.result)) };
        write!(f, "{}", raw_message.to_string_lossy())
    }
}

impl std::fmt::Debug for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Maybe use PQresultVerboseErrorMessage here?
        write!(f, "{self}")
    }
}

impl std::error::Error for QueryError {}

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
    fn fetch_cell<'a>(&self, row: usize, column: usize) -> Option<&'a [u8]> {
        let row: i32 = row as i32;
        let column: i32 = column as i32;
        unsafe {
            // Pointer postgres gives is signed bytes, but rust wants raw data to be unsigned bytes. It's the same bytes though.
            let data: *mut u8 = libpq::PQgetvalue(self.result, row, column) as *mut u8;
            if libpq::PQgetisnull(self.result, row, column) == 1 {
                None
            } else {
                let len: usize = libpq::PQgetlength(self.result, row, column) as usize;
                Some(std::slice::from_raw_parts::<'a>(data, len))
            }
        }
    }

    pub fn fetch_one<
        'a,
        const N: usize,
        T: std::convert::From<[std::option::Option<&'a [u8]>; N]>,
    >(
        &self,
    ) -> T {
        assert!(N <= unsafe { libpq::PQnfields(self.result) } as usize);
        self.fetch_one_unchecked()
    }

    /// Fetches the first N columns of a single row of the query result.
    fn fetch_one_unchecked<
        'a,
        const N: usize,
        T: std::convert::From<[std::option::Option<&'a [u8]>; N]>,
    >(
        &self,
    ) -> T {
        core::array::from_fn(|column| self.fetch_cell(0, column)).into()
    }

    /// Fetches all first N columns and all the rows of the query result.
    pub fn fetch_all<
        'a,
        const N: usize,
        T: std::convert::From<[std::option::Option<&'a [u8]>; N]>,
    >(
        &self,
    ) -> Vec<T> {
        assert!(N <= unsafe { libpq::PQnfields(self.result) as usize });
        let row_count: usize = unsafe { libpq::PQntuples(self.result) } as usize;
        let mut result: Vec<T> = Vec::with_capacity(row_count);
        for _ in 0..N {
            result.push(self.fetch_one_unchecked());
        }
        result
    }
}

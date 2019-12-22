use reindexer_sys::ffi::{self};
use crate::iter::Iter;

pub struct QueryResults {
    pub inner: *mut ffi::QueryResults,
}

impl QueryResults {
    pub fn new() -> Self {
        QueryResults {
            inner: unsafe { ffi::re_query_results_new() },
        }
    }

    pub fn count(&mut self) -> i32 {
        unsafe { ffi::re_query_results_count(self.inner) }
    }

    pub fn iter(&mut self) -> Iter {
        let inner = unsafe { ffi::re_query_results_iter(self.inner) };
        Iter { inner }
    }
}

impl Drop for QueryResults {
    fn drop(&mut self) {
        unsafe {
            ffi::re_query_results_destroy(self.inner);
        }
    }
}
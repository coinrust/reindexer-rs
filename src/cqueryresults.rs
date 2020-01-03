use reindexer_sys::ffi::{self};
use crate::citer::CIter;

pub struct CQueryResults {
    pub inner: *mut ffi::CQueryResults,
}

impl CQueryResults {
    pub fn new() -> Self {
        CQueryResults {
            inner: unsafe { ffi::re_client_query_results_new() },
        }
    }

    pub fn count(&self) -> i32 {
        unsafe { ffi::re_client_query_results_count(self.inner) }
    }

    pub fn iter(&self) -> CIter {
        let inner = unsafe { ffi::re_client_query_results_iter(self.inner) };
        CIter { inner }
    }
}

impl Drop for CQueryResults {
    fn drop(&mut self) {
        unsafe {
            ffi::re_client_query_results_destroy(self.inner);
        }
    }
}
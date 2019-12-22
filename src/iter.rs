use reindexer_sys::ffi::{self};
use std::ffi::CStr;

pub struct Iter {
    pub inner: *mut ffi::QueryResultsIterator,
}

impl Iter {
    pub fn next(&mut self) -> bool {
        unsafe { ffi::re_query_results_iter_next(self.inner) }
    }

    pub fn get_json(&mut self) -> String {
        unsafe {
            let str_buff = ffi::re_query_results_iter_get_json(self.inner);
            let cstr = CStr::from_ptr(str_buff);
            cstr.to_string_lossy().into_owned()
        }
    }
}

impl Drop for Iter {
    fn drop(&mut self) {
        unsafe {
            ffi::re_query_results_iter_destroy(self.inner);
        }
    }
}
use reindexer_sys::ffi::{self};
use std::ffi::CStr;

pub struct CIter {
    pub inner: *mut ffi::CQueryResultsIterator,
}

impl CIter {
    pub fn next(&mut self) -> bool {
        unsafe { ffi::re_client_query_results_iter_next(self.inner) }
    }

    pub fn get_json(&self) -> String {
        unsafe {
            let str_buff = ffi::re_client_query_results_iter_get_json(self.inner);
            let cstr = CStr::from_ptr(str_buff);
            cstr.to_string_lossy().into_owned()
        }
    }
}

impl Iterator for CIter {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.next() {
            Some(self.get_json())
        } else {
            None
        }
    }
}

impl Drop for CIter {
    fn drop(&mut self) {
        unsafe {
            ffi::re_client_query_results_iter_destroy(self.inner);
        }
    }
}
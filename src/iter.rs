use reindexer_sys::ffi::{self};
use std::ffi::CString;

pub struct Iter {
    pub inner: *mut ffi::Iterator,
}

impl Iter {
    pub fn next(&mut self) -> bool {
        unsafe { ffi::re_client_query_results_iter_next(self.inner) }
    }

    pub fn get_json(&mut self) -> String {
        let output = CString::new("").unwrap();
        let raw = output.into_raw();
        let ok = unsafe { ffi::re_client_query_results_iter_get_json(self.inner, raw) };
        if !ok {
            return String::from("");
        }
        let output = unsafe { CString::from_raw(raw) };
        let s = output.to_str().unwrap();
        s.to_owned()
    }
}

impl Drop for Iter {
    fn drop(&mut self) {
        unsafe {
            ffi::re_client_query_results_iter_destroy(self.inner);
        }
    }
}
use reindexer_sys::ffi::{self};
use std::ffi::CString;
use crate::queryresults::QueryResults;

pub struct Reindexer {
    inner: *mut ffi::Reindexer,
}

impl Reindexer {
    pub fn new() -> Self {
        Reindexer {
            inner: unsafe { ffi::re_new() },
        }
    }

    pub fn open_namespace(&mut self, ns: &str, storage_enabled: bool) -> bool {
        let ns = CString::new(ns).unwrap();
        unsafe {
            ffi::re_open_namespace(self.inner, ns.as_ptr(), storage_enabled)
        }
    }

    pub fn add_index(&mut self, ns: &str, name: &str, index_type: &str, field_type: &str, pk: bool) -> bool {
        let ns = CString::new(ns).unwrap();
        let name = CString::new(name).unwrap();
        let index_type = CString::new(index_type).unwrap();
        let field_type = CString::new(field_type).unwrap();
        let index_opts = unsafe { ffi::index_opts_new() };
        if pk {
            unsafe { ffi::index_opts_pk(index_opts); }
        }
        let ok = unsafe {
            ffi::re_add_index(self.inner, ns.as_ptr(), name.as_ptr(), index_type.as_ptr(), field_type.as_ptr(), index_opts)
        };
        unsafe { ffi::index_opts_destroy(index_opts); };
        ok
    }

    /*
    data: `{"id":1,"name":"test"}`
    */
    pub fn insert(&mut self, ns: &str, data: &str) -> bool {
        // r#"{"id":1234, "value" : "value"}"#
        let ns = CString::new(ns).unwrap();
        let data = CString::new(data).unwrap();
        unsafe { ffi::re_insert(self.inner, ns.as_ptr(), data.as_ptr()) }
    }

    /*
    data: `{"id":1,"name":"test"}`
    */
    pub fn upsert(&mut self, ns: &str, data: &str) -> bool {
        // r#"{"id":1234, "value" : "value"}"#
        let ns = CString::new(ns).unwrap();
        let data = CString::new(data).unwrap();
        unsafe { ffi::re_upsert(self.inner, ns.as_ptr(), data.as_ptr()) }
    }

    /*
    data: `{"id":1,"name":"test"}`
    */
    pub fn update(&mut self, ns: &str, data: &str) -> bool {
        // r#"{"id":1234, "value" : "value"}"#
        let ns = CString::new(ns).unwrap();
        let data = CString::new(data).unwrap();
        unsafe { ffi::re_update(self.inner, ns.as_ptr(), data.as_ptr()) }
    }

    /*
    data: `{"id":1}`
    */
    pub fn delete(&mut self, ns: &str, data: &str) -> bool {
        // r#"{"id":1234, "value" : "value"}"#
        let ns = CString::new(ns).unwrap();
        let data = CString::new(data).unwrap();
        unsafe { ffi::re_delete(self.inner, ns.as_ptr(), data.as_ptr()) }
    }

    /*
    query: `"SELECT * FROM items"`
    */
    pub fn select(&mut self, query: &str) -> (QueryResults, bool) {
        let query = CString::new(query).unwrap();
        let qr = QueryResults::new();
        let ok = unsafe { ffi::re_select(self.inner, qr.inner, query.as_ptr()) };
        (qr, ok)
    }
}

impl Drop for Reindexer {
    fn drop(&mut self) {
        unsafe {
            ffi::re_destroy(self.inner);
        }
    }
}
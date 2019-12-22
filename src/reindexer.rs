use reindexer_sys::ffi::{self};
use std::ffi::CString;
use crate::query_results::CQueryResults;

pub struct CReindexer {
    inner: *mut ffi::CReindexer,
}

impl CReindexer {
    pub fn new() -> Self {
        CReindexer {
            inner: unsafe { ffi::re_client_new() },
        }
    }

    pub fn connect(&mut self, dsn: &str) -> bool {
        let dsn = CString::new(dsn).unwrap();
        unsafe {
            ffi::re_client_connect(self.inner, dsn.as_ptr())
        }
    }

    pub fn open_namespace(&mut self, ns: &str) -> bool {
        let ns = CString::new(ns).unwrap();
        unsafe {
            ffi::re_client_open_namespace(self.inner, ns.as_ptr())
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
            ffi::re_client_add_index(self.inner, ns.as_ptr(), name.as_ptr(), index_type.as_ptr(), field_type.as_ptr(), index_opts)
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
        unsafe { ffi::re_client_insert(self.inner, ns.as_ptr(), data.as_ptr()) }
    }

    /*
    data: `{"id":1,"name":"test"}`
    */
    pub fn upsert(&mut self, ns: &str, data: &str) -> bool {
        // r#"{"id":1234, "value" : "value"}"#
        let ns = CString::new(ns).unwrap();
        let data = CString::new(data).unwrap();
        unsafe { ffi::re_client_upsert(self.inner, ns.as_ptr(), data.as_ptr()) }
    }

    /*
    data: `{"id":1,"name":"test"}`
    */
    pub fn update(&mut self, ns: &str, data: &str) -> bool {
        // r#"{"id":1234, "value" : "value"}"#
        let ns = CString::new(ns).unwrap();
        let data = CString::new(data).unwrap();
        unsafe { ffi::re_client_update(self.inner, ns.as_ptr(), data.as_ptr()) }
    }

    /*
    data: `{"id":1}`
    */
    pub fn delete(&mut self, ns: &str, data: &str) -> bool {
        // r#"{"id":1234, "value" : "value"}"#
        let ns = CString::new(ns).unwrap();
        let data = CString::new(data).unwrap();
        unsafe { ffi::re_client_delete(self.inner, ns.as_ptr(), data.as_ptr()) }
    }

    /*
    query: `"SELECT * FROM items"`
    */
    pub fn select(&mut self, query: &str) -> (CQueryResults, bool) {
        let query = CString::new(query).unwrap();
        let qr = CQueryResults::new();
        let ok = unsafe { ffi::re_client_select(self.inner, qr.inner, query.as_ptr()) };
        (qr, ok)
    }
}

impl Drop for CReindexer {
    fn drop(&mut self) {
        unsafe {
            ffi::re_client_destroy(self.inner);
        }
    }
}
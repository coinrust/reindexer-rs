use crate::queryresults::QueryResults;
use reindexer_sys::ffi::{self};
use std::ffi::CString;

pub struct Reindexer {
    inner: *mut ffi::Reindexer,
}

unsafe impl Send for Reindexer {}
unsafe impl Sync for Reindexer {}

impl Reindexer {
    pub fn new() -> Self {
        Reindexer {
            inner: unsafe { ffi::re_new() },
        }
    }

    pub fn connet(&self, dsn: &str) {
        let dsn = CString::new(dsn).unwrap();
        unsafe { ffi::re_connect(self.inner, dsn.as_ptr()) }
    }

    pub fn open_namespace(&self, ns: &str) -> bool {
        let ns = CString::new(ns).unwrap();
        unsafe { ffi::re_open_namespace(self.inner, ns.as_ptr()) }
    }

    /// add_index
    /// ns
    /// name
    /// json_paths: `` or `id,fk_id` for composite fields
    /// index_type: hash, tree, text, fuzzytext, -
    /// field_type: int, int64, string, composite, double, bool
    /// pk
    pub fn add_index(
        &self,
        ns: &str,
        name: &str,
        json_paths: &str,
        index_type: &str,
        field_type: &str,
        pk: bool,
    ) -> bool {
        let ns = CString::new(ns).unwrap();
        let name = CString::new(name).unwrap();
        let json_paths = CString::new(json_paths).unwrap();
        let index_type = CString::new(index_type).unwrap();
        let field_type = CString::new(field_type).unwrap();
        let index_opts = unsafe { ffi::index_opts_new() };
        if pk {
            unsafe {
                ffi::index_opts_pk(index_opts);
            }
        }
        let ok = unsafe {
            ffi::re_add_index(
                self.inner,
                ns.as_ptr(),
                name.as_ptr(),
                json_paths.as_ptr(),
                index_type.as_ptr(),
                field_type.as_ptr(),
                index_opts,
            )
        };
        unsafe {
            ffi::index_opts_destroy(index_opts);
        };
        ok
    }

    /// add_index_from_json
    /// ns
    /// index_def_json:
    /// {"name":"id","field_type":"int","index_type":"hash","is_pk":false,"is_array":false,"is_dense":false,"is_sparse":false,"collate_mode":"none","sort_order_letters":"","expire_after":0,"config":{},"json_paths":["id"]}
    /// {"name":"id+uid","field_type":"composite","index_type":"tree","is_pk":true,"is_array":false,"is_dense":false,"is_sparse":false,"collate_mode":"none","sort_order_letters":"","expire_after":0,"config":{},"json_paths":["id","uid"]}
    pub fn add_index_from_json(&self, ns: &str, index_def_json: &str) -> bool {
        let ns = CString::new(ns).unwrap();
        let index_def_json = CString::new(index_def_json).unwrap();
        let ok = unsafe { ffi::re_add_index_from_json(self.inner, ns.as_ptr(), index_def_json.as_ptr()) };
        ok
    }

    /*
    data: `{"id":1,"name":"test"}`
    */
    pub fn insert(&self, ns: &str, data: &str) -> bool {
        // r#"{"id":1234, "value" : "value"}"#
        let ns = CString::new(ns).unwrap();
        let data = CString::new(data).unwrap();
        unsafe { ffi::re_insert(self.inner, ns.as_ptr(), data.as_ptr()) }
    }

    /*
    data: `{"id":1,"name":"test"}`
    */
    pub fn upsert(&self, ns: &str, data: &str) -> bool {
        // r#"{"id":1234, "value" : "value"}"#
        let ns = CString::new(ns).unwrap();
        let data = CString::new(data).unwrap();
        unsafe { ffi::re_upsert(self.inner, ns.as_ptr(), data.as_ptr()) }
    }

    /*
    data: `{"id":1,"name":"test"}`
    */
    pub fn update(&self, ns: &str, data: &str) -> bool {
        // r#"{"id":1234, "value" : "value"}"#
        let ns = CString::new(ns).unwrap();
        let data = CString::new(data).unwrap();
        unsafe { ffi::re_update(self.inner, ns.as_ptr(), data.as_ptr()) }
    }

    /*
    data: `{"id":1}`
    */
    pub fn delete(&self, ns: &str, data: &str) -> bool {
        // r#"{"id":1234, "value" : "value"}"#
        let ns = CString::new(ns).unwrap();
        let data = CString::new(data).unwrap();
        unsafe { ffi::re_delete(self.inner, ns.as_ptr(), data.as_ptr()) }
    }

    /*
    query: `"SELECT * FROM items"`
    */
    pub fn select(&self, query: &str) -> (QueryResults, bool) {
        let query = CString::new(query).unwrap();
        let qr = QueryResults::new();
        let ok = unsafe { ffi::re_select(self.inner, qr.inner, query.as_ptr()) };
        (qr, ok)
    }

    /*
    query: `"UPDATE items SET value = 'value1' WHERE id = 1000"`
    */
    pub fn update_sql(&self, query: &str) -> (QueryResults, bool) {
        let query = CString::new(query).unwrap();
        let qr = QueryResults::new();
        let ok = unsafe { ffi::re_update_sql(self.inner, qr.inner, query.as_ptr()) };
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

use std::os::raw::{c_char, c_int};

// https://rustcc.gitbooks.io/rustprimer/content/ffi/calling-ffi-function.html

pub enum Reindexer {} // reindexer::client::Reindexer
pub enum QueryResults {} // reindexer::client::QueryResults
pub enum Iterator {} // reindexer::client::QueryResults::Iterator
//pub enum WrSerializer {} // WrSerializer
//pub enum Error {} // reindexer::Error

#[allow(dead_code)]
extern "C" {
    pub fn re_test() -> ();
    pub fn re_client_test() -> ();

    pub fn re_client_new() -> *mut Reindexer;
    pub fn re_client_destroy(db: *mut Reindexer) -> ();
    pub fn re_client_connect(db: *mut Reindexer, dsn: *const c_char) -> bool;
    pub fn re_client_open_namespace(db: *mut Reindexer, ns: *const c_char) -> bool;
    
    pub fn re_client_insert(db: *mut Reindexer, ns: *const c_char, data: *const c_char) -> bool;
    pub fn re_client_update(db: *mut Reindexer, ns: *const c_char, data: *const c_char) -> bool;
    pub fn re_client_upsert(db: *mut Reindexer, ns: *const c_char, data: *const c_char) -> bool;
    pub fn re_client_delete(db: *mut Reindexer, ns: *const c_char, data: *const c_char) -> bool;
    pub fn re_client_select(db: *mut Reindexer, qr: *mut QueryResults, query: *const c_char) -> bool;
    
    pub fn re_client_query_results_new() -> *mut QueryResults;
    pub fn re_client_query_results_destroy(qr: *mut QueryResults) -> ();
    
    pub fn re_client_query_results_count(qr: *mut QueryResults) -> c_int;
    pub fn re_client_query_results_iter(qr: *mut QueryResults) -> *mut Iterator;
    pub fn re_client_query_results_iterator_next(it: *mut Iterator) -> bool;
    pub fn re_client_query_results_iter_destroy(it: *mut Iterator) -> ();
    pub fn re_client_query_results_iter_get_json(it: *mut Iterator, output: *mut c_char) -> bool;

    //pub fn wr_serializer_new() -> *mut WrSerializer;
    //pub fn wr_serializer_destroy(qr: *mut WrSerializer) -> ();
}
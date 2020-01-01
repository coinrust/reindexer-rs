use std::os::raw::{c_char, c_int};

// https://rustcc.gitbooks.io/rustprimer/content/ffi/calling-ffi-function.html

pub enum CReindexer {} // reindexer::client::Reindexer
pub enum CQueryResults {} // reindexer::client::QueryResults
pub enum CQueryResultsIterator {} // reindexer::client::QueryResults::Iterator
pub enum IndexOpts {} // IndexOpts

pub enum Reindexer {} // reindexer::Reindexer
pub enum QueryResults {} // reindexer::QueryResults
pub enum QueryResultsIterator {} // reindexer::QueryResults::Iterator

#[allow(dead_code)]
extern "C" {
    pub fn re_test() -> ();
    pub fn re_client_test() -> ();

    pub fn re_client_new() -> *mut CReindexer;
    pub fn re_client_destroy(db: *mut CReindexer) -> ();
    pub fn re_client_connect(db: *mut CReindexer, dsn: *const c_char) -> bool;
    pub fn re_client_open_namespace(db: *mut CReindexer, ns: *const c_char, storage_enabled: bool) -> bool;

    pub fn index_opts_new() -> *mut IndexOpts;
    pub fn index_opts_destroy(indexOpts: *mut IndexOpts) -> ();
    pub fn index_opts_pk(indexOpts: *mut IndexOpts) -> ();

    pub fn re_client_add_index(db: *mut CReindexer, ns: *const c_char, name: *const c_char,
                               indexType: *const c_char, fieldType: *const c_char, indexOpts: *mut IndexOpts) -> bool;

    pub fn re_client_insert(db: *mut CReindexer, ns: *const c_char, data: *const c_char) -> bool;
    pub fn re_client_update(db: *mut CReindexer, ns: *const c_char, data: *const c_char) -> bool;
    pub fn re_client_upsert(db: *mut CReindexer, ns: *const c_char, data: *const c_char) -> bool;
    pub fn re_client_delete(db: *mut CReindexer, ns: *const c_char, data: *const c_char) -> bool;
    pub fn re_client_select(db: *mut CReindexer, qr: *mut CQueryResults, query: *const c_char) -> bool;
    
    pub fn re_client_query_results_new() -> *mut CQueryResults;
    pub fn re_client_query_results_destroy(qr: *mut CQueryResults) -> ();
    
    pub fn re_client_query_results_count(qr: *mut CQueryResults) -> c_int;
    pub fn re_client_query_results_iter(qr: *mut CQueryResults) -> *mut CQueryResultsIterator;
    pub fn re_client_query_results_iter_next(it: *mut CQueryResultsIterator) -> bool;
    pub fn re_client_query_results_iter_destroy(it: *mut CQueryResultsIterator) -> ();
    pub fn re_client_query_results_iter_get_json(it: *mut CQueryResultsIterator) -> *mut c_char;

    /// **** ///
    pub fn re_new() -> *mut Reindexer;
    pub fn re_destroy(db: *mut Reindexer) -> ();
    pub fn re_connect(db: *mut Reindexer, dsn: *const c_char) -> ();
    pub fn re_open_namespace(db: *mut Reindexer, ns: *const c_char) -> bool;

    pub fn re_add_index(db: *mut Reindexer, ns: *const c_char, name: *const c_char,
                               indexType: *const c_char, fieldType: *const c_char, indexOpts: *mut IndexOpts) -> bool;

    pub fn re_insert(db: *mut Reindexer, ns: *const c_char, data: *const c_char) -> bool;
    pub fn re_update(db: *mut Reindexer, ns: *const c_char, data: *const c_char) -> bool;
    pub fn re_upsert(db: *mut Reindexer, ns: *const c_char, data: *const c_char) -> bool;
    pub fn re_delete(db: *mut Reindexer, ns: *const c_char, data: *const c_char) -> bool;
    pub fn re_select(db: *mut Reindexer, qr: *mut QueryResults, query: *const c_char) -> bool;

    pub fn re_query_results_new() -> *mut QueryResults;
    pub fn re_query_results_destroy(qr: *mut QueryResults) -> ();

    pub fn re_query_results_count(qr: *mut QueryResults) -> c_int;
    pub fn re_query_results_iter(qr: *mut QueryResults) -> *mut QueryResultsIterator;
    pub fn re_query_results_iter_next(it: *mut QueryResultsIterator) -> bool;
    pub fn re_query_results_iter_destroy(it: *mut QueryResultsIterator) -> ();
    pub fn re_query_results_iter_get_json(it: *mut QueryResultsIterator) -> *mut c_char;
}
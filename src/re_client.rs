use reindexer_sys::ffi::{self};

pub struct DB {
    inner: *mut Reindexer,
}
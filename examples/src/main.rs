use reindexer_sys::ffi;
use std::ffi::CString;

fn main() {
    unsafe {
        let client = ffi::re_client_new();

        let dsn = CString::new(format!("{}", "cproto://127.0.0.1:6534/test_db")).unwrap();
        let ok = ffi::re_client_connect(client, dsn.as_ptr());
        println!("re_client_connect: {}", ok);

        let ns = CString::new("test_namespace").unwrap();
        let ok = ffi::re_client_open_namespace(client, ns.as_ptr());
        println!("re_client_open_namespace: {}", ok);

        let data = CString::new(r#"{"id":1234, "value" : "value"}"#).unwrap();
        let ok = ffi::re_client_insert(client, ns.as_ptr(), data.as_ptr());
        println!("re_client_insert: {}", ok);

        let data = CString::new(r#"{"id":1235, "value" : "value"}"#).unwrap();
        let ok = ffi::re_client_insert(client, ns.as_ptr(), data.as_ptr());
        println!("re_client_insert: {}", ok);

        let qr = ffi::re_client_query_results_new();
        let query = CString::new("SELECT * FROM test_namespace").unwrap();
        let ok = ffi::re_client_select(client, qr, query.as_ptr());
        println!("re_client_select: {}", ok);

        let count = ffi::re_client_query_results_count(qr);
        println!("re_client_query_results_count: {}", count);

        let it = ffi::re_client_query_results_iter(qr);

        loop {
            let ok = ffi::re_client_query_results_iterator_next(it);
            if !ok {
                break;
            }
            let output = CString::new("").unwrap();
            let raw = output.into_raw();
            let ok = ffi::re_client_query_results_iter_get_json(it, raw);
            println!("re_client_query_results_iter_get_json ok: {}", ok);
            let output2 = CString::from_raw(raw);
            println!("re_client_query_results_iter_get_json: item -> {:?}", output2.to_string_lossy());
        }

        ffi::re_client_query_results_iter_destroy(it);
        println!("re_client_query_results_iter_destroy");

        ffi::re_client_query_results_destroy(qr);
        println!("re_client_query_results_destroy");

        ffi::re_client_destroy(client);
        println!("re_client_destroy");

        //ffi::re_test();
        //ffi::re_client_test();
    }

    println!("OK");
}

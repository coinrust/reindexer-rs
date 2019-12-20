# reindexer-rs
Reindexer library for Rust

# Build
```
$ cargo build
```

# Run
```
$ cargo run -p reindexer-examples
```

# Code
```
let mut db = Reindexer::new();
let ok = db.connect("cproto://127.0.0.1:6534/test_db");
assert_eq!(true, ok);

let ns = "items";
let ok = db.open_namespace(ns);
assert_eq!(true, ok);

let ok = db.add_index(ns, "id", "hash", "int", true);
assert_eq!(true, ok);

let item = r#"{"id":1234, "value" : "value"}"#;
let ok = db.upsert(ns, item);
assert_eq!(true, ok);

let item = r#"{"id":1235, "value" : "value"}"#;
let ok = db.upsert(ns, item);
assert_eq!(true, ok);

let (mut qr, ok) = db.select("SELECT * FROM items");
assert_eq!(true, ok);

let mut it = qr.iter();
loop {
    let ok = it.next();
    if !ok {
        break;
    }
    let json = it.get_json();
    println!("item: {}", json);
}
```
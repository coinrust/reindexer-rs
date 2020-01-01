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

# Example 1
```rust,editable
// builtin
let mut db = Reindexer::new();

db.connet("builtin:///tmp/reindex/testdb");

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

let (_, ok) = db.update_sql("UPDATE items SET ext = 'hello' WHERE id = 1235");
assert_eq!(true, ok);

let (mut qr, ok) = db.select("SELECT * FROM items WHERE id = 1235");
assert_eq!(true, ok);

for s in qr.iter() {
    println!("item: {}", s);
}
```

# Example 2
```rust,editable
// cproto
let mut db = CReindexer::new();
let ok = db.connect("cproto://127.0.0.1:6534/test_db");
assert_eq!(true, ok);

let ns = "items";
let ok = db.open_namespace(ns, true);
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

for s in qr.iter() {
    println!("item: {}", s);
}
```
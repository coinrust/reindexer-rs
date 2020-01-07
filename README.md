# reindexer-rs
Reindexer library for Rust

# Installation (refer to [installation](https://github.com/Restream/reindexer/blob/master/cpp_src/readme.md#installation))
```
go get github.com/restream/reindexer
sudo apt-get install libgoogle-perftools-dev
cd $GOPATH/src/github.com/restream/reindexer
sudo ./dependencies.sh
mkdir -p build && cd build
cmake ..
make -j4
# optional: step for build swagger documentation
make swagger
# optional: step for build web pages of Reindexer's face
make face
# install to system
sudo make install
```

# Build
```
# Build
$ cargo build

# Build examples
$ cargo build -p reindexer-examples
```

# Example (builtin)
```rust,editable
// builtin
let db = Reindexer::new();

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

let (qr, ok) = db.select("SELECT * FROM items WHERE id = 1235");
assert_eq!(true, ok);

for s in qr.iter() {
    println!("item: {}", s);
}
```

# Example (cproto)
```rust,editable
// cproto
let db = CReindexer::new();
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

let (qr, ok) = db.select("SELECT * FROM items");
assert_eq!(true, ok);

for s in qr.iter() {
    println!("item: {}", s);
}
```
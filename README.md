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
$ cargo build --examples

# Run hello.rs
$ cargo run --example hello
```

# Example (builtin)
```rust,editable
// builtin
let db = Reindexer::new();

db.connet("builtin:///tmp/reindex/testdb");

let ns = "items";
let ok = db.open_namespace(ns);
assert!(ok);

let ok = db.add_index(ns, "id", "", "hash", "int", false);
assert!(ok);

let ok = db.add_index(ns, "fk_id", "", "hash", "int", false);
assert!(ok);

let ok = db.add_index(ns, "id+fk_id", "id,fk_id", "tree", "composite", true);
assert!(ok);

let item = r#"{"id":1234, "value" : "value"}"#;
let ok = db.upsert(ns, item);
assert!(ok);

let item = r#"{"id":1235, "value" : "value"}"#;
let ok = db.upsert(ns, item);
assert!(ok);

let (_, ok) = db.update_sql("UPDATE items SET ext = 'hello' WHERE id = 1235");
assert!(ok);

let (qr, ok) = db.select("SELECT * FROM items WHERE id = 1235");
assert!(ok);

let mut n = 0;
for s in qr.iter() {
    println!("item: {}", s);
    n += 1;
    if n > 10 {
        break;
    }
}
```

# Example (cproto)
```rust,editable
// cproto
let db = CReindexer::new();
let ok = db.connect("cproto://127.0.0.1:6534/test_db");
assert!(ok);

let ns = "items";
let ok = db.open_namespace(ns, true);
assert!(ok);

let ok = db.add_index(ns, "id", "hash", "int", true);
assert!(ok);

let item = r#"{"id":1234, "value" : "value"}"#;
let ok = db.upsert(ns, item);
assert!(ok);

let item = r#"{"id":1235, "value" : "value"}"#;
let ok = db.upsert(ns, item);
assert!(ok);

let (qr, ok) = db.select("SELECT * FROM items");
assert!(ok);

for s in qr.iter() {
    println!("item: {}", s);
}
```
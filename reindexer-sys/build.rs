extern crate bindgen;
extern crate cc;

use std::env;
use std::path::Path;

fn main() {
    let go_path = env::var("GOPATH").unwrap();

    let reindexer_path = Path::new( &go_path ).join("src/github.com/restream/reindexer/cpp_src").display().to_string();
    let reindexer_build_path = Path::new( &go_path ).join("src/github.com/restream/reindexer/build/cpp_src").display().to_string();
    let reindexer_vendor_path = Path::new( &go_path ).join("src/github.com/restream/reindexer/cpp_src/vendor").display().to_string();

    cc::Build::new()
        .cpp(true)
        .file("src/ffi.cpp")
        .flag("-x")
        .flag("c++")
        .flag("-std=c++11")
        .include(&reindexer_vendor_path)
        .include(&reindexer_path)
        .include("src")
        .compile("reindexer_ffi");

    println!("cargo:rustc-link-search=native={}", &reindexer_build_path);
    println!("cargo:rustc-link-lib=static=reindexer_ffi");
    println!("cargo:rustc-link-lib=dylib=snappy");
    println!("cargo:rustc-link-lib=dylib=leveldb");
    println!("cargo:rustc-link-lib=static=reindexer");

    println!("cargo:rerun-if-changed=src/*");
    println!("cargo:rerun-if-changed=build.rs");
}

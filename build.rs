//! Rust FFI Template

use std::env::var;
use std::process::Command;

fn main() {
    Command::new("./tools/build_c.sh").spawn().unwrap();

    println!("cargo:rustc-link=library");
    println!(
        "cargo:rustc-link-search={}/src/ffi/build/",
        var("CARGO_MANIFEST_DIR").unwrap()
    );
}

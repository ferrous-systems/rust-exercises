//! # Build script for the QEMU Ferrocene demo project
//!
//! This script only executes when using `cargo` to build the project.

use std::io::Write;

fn main() {
    // Put `memory.x` file in our output directory and ensure it's on the
    // linker search path.
    let out = &std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    // We need to link with this file, which comes from cortex-r-rt.
    println!("cargo:rustc-link-arg=-Tlink.x");
}

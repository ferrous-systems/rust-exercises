//! # Build script for the QEMU Ferrocene demo project
//!
//! This script only executes when using `cargo` to build the project.

use std::io::Write;

fn main() {
    // Find the right tools.
    let linker = std::env::var("RUSTC_LINKER");
    let linker = linker.as_deref().unwrap_or("arm-none-eabi-gcc");
    let arm_as = linker.replace("gcc", "as");
    let arm_ar = linker.replace("gcc", "ar");

    // Put `linker.ld` file in our output directory and ensure it's on the
    // linker search path.
    let out = &std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::File::create(out.join("linker.ld"))
        .unwrap()
        .write_all(include_bytes!("linker.ld"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // Assemble src/boot.S
    let boot_object = out.join("boot.o");
    let output = std::process::Command::new(&arm_as)
        .arg("src/boot.S")
        .arg("-march=armv8-r")
        .arg("-mfpu=fp-armv8")
        .arg("-o")
        .arg(&boot_object)
        .output()
        .map(|h| h.status.success());
    match output {
        Ok(true) => {
            // Ran OK
        }
        Ok(false) => {
            // Didn't launch
            panic!("Failed to launch {arm_as}");
        }
        Err(e) => {
            // Failed to run
            panic!("Failed to run {arm_as}: {e:?}");
        }
    }

    // Place assembled object code into a static library
    let libboot_file = out.join("libboot.a");
    let output = std::process::Command::new(&arm_ar)
        .arg("rcs")
        .arg(&libboot_file)
        .arg(&boot_object)
        .output()
        .map(|h| h.status.success());
    match output {
        Ok(true) => {
            // Ran OK
        }
        Ok(false) => {
            // Didn't launch
            panic!("Failed to launch {arm_ar}");
        }
        Err(e) => {
            // Failed to run
            panic!("Failed to run {arm_ar}: {e:?}");
        }
    }

    // Tell cargo to link against our new libboot.a library
    println!("cargo:rustc-link-lib=static=boot");
}

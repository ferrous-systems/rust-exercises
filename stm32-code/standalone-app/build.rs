//! # Build script for nonsecure-app

fn main() {
    // Find a place to put our temporary files
    let out = &std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    // Copy our linker scripts to that temporary place (leaving them in the current working
    // directory doesn't work if someone does an out-of-tree build)
    std::fs::write(out.join("memory.x"), include_bytes!("memory.x")).unwrap();
    // Look for memory.x in the output dir (it includes stm32u5a5.x)
    println!("cargo:rustc-link-search={}", out.display());
    // Rebuild if our linker scripts change
    println!("cargo::rerun-if-changed=memory.x");
    // Use cortex-m-rt's link.x as the linker script
    println!("cargo:rustc-link-arg=-Tlink.x");
    // We need the defmt linker script to put the log messages in a NOLOAD section
    println!("cargo:rustc-link-arg=-Tdefmt.x");
    // Import the file where the symbol table notes where the Secure Gateway stubs are
    println!("cargo:rustc-link-arg=target/libsec_bootloader_stubs.o");
    // And rebuild our program if it changes
    println!("cargo::rerun-if-changed=target/libsec_bootloader_stubs.o");
}

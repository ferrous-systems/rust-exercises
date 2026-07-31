//! # Build script for secure-loader

fn main() {
    // Find a place to put our temporary files
    let out = &std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    // Copy our linker scripts to that temporary place (leaving them in the current working
    // directory doesn't work if someone does an out-of-tree build)
    std::fs::write(out.join("memory.x"), include_bytes!("memory.x")).unwrap();
    // Look for memory.x in the output dir
    println!("cargo:rustc-link-search={}", out.display());
    // Rebuild if our linker scripts change
    println!("cargo::rerun-if-changed=memory.x");
    // Use cortex-m-rt's link.x as the linker script
    println!("cargo:rustc-link-arg=-Tlink.x");
    // We are using Cortex-M Security Extensions (aka TrustZone)
    println!("cargo:rustc-link-arg=--cmse-implib");
    // Use top of Bank 0 for the Secure Gateway stubs
    println!("cargo:rustc-link-arg=--section-start=.gnu.sgstubs=0x0C1F0000");
    // Emit a file where the symbol table notes where the Secure Gateway stubs are
    println!("cargo:rustc-link-arg=--out-implib=target/libsec_bootloader_stubs.o");
}

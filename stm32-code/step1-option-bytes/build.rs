//! # Build script for step1-option-bytes

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
}

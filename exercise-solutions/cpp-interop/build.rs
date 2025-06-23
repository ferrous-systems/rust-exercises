fn main() -> miette::Result<()> {
    let include_path = std::path::PathBuf::from("src");

    // This assumes all your C++ bindings are in main.rs
    let mut b = autocxx_build::Builder::new("src/main.rs", [&include_path]).build()?;
    b.flag_if_supported("-std=c++20").compile("autocxx-demo"); // arbitrary library name, pick anything
    println!("cargo:rerun-if-changed=src/main.rs");

    // Add instructions to link to any C++ libraries you need.

    Ok(())
}

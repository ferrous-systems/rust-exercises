fn main() {
    if std::env::var("SECRET_MESSAGE").is_err() {
        println!("cargo:rustc-env=SECRET_MESSAGE=WELLDONE");
    }
    if std::env::var("PLAIN_LETTERS").is_err() {
        println!("cargo:rustc-env=PLAIN_LETTERS=abcdefghijklmnopqrstuvwxyz");
    }
    if std::env::var("CIPHER_LETTERS").is_err() {
        println!("cargo:rustc-env=CIPHER_LETTERS=ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
}

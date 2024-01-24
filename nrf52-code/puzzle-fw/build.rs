//! Configure the puzzle firmware

use rand::prelude::*;

fn main() {
    // We avoid \ to prevent escaping issues
    const PLAIN_LETTERS: &str = r##"0123456789 abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!"#$%&'()*+,-./:;<=>?@[\]^_`{|}~"##;

    let maybe_msg = std::env::var("HIDDEN_MESSAGE");
    let plaintext = maybe_msg.as_deref().unwrap_or("This is an example message");

    let mut rng = rand::thread_rng();
    let mut cipher_letters: Vec<u8> = PLAIN_LETTERS.bytes().collect();
    cipher_letters.shuffle(&mut rng);
    let cipher_letters_str = std::str::from_utf8(&cipher_letters).unwrap();

    let mut dict = std::collections::HashMap::new();
    for (from, &to) in PLAIN_LETTERS.bytes().zip(cipher_letters.iter()) {
        dict.insert(from, to);
    }

    println!("from: {:?}", PLAIN_LETTERS);
    println!("to: {:?}", cipher_letters_str);
    println!("plaintext: {:?}", plaintext);

    let encoded: Vec<u8> = plaintext.bytes().map(|byte| dict[&byte]).collect();
    let encoded_str = std::str::from_utf8(&encoded).unwrap();
    println!("secret: {:?}", encoded_str);

    output_data("ENCODED_MESSAGE.txt", encoded_str);
    output_data("PLAIN_LETTERS.txt", PLAIN_LETTERS);
    output_data("CIPHER_LETTERS.txt", cipher_letters_str);

    println!("cargo:rerun-if-env-changed=HIDDEN_MESSAGE");
}

fn output_data(filename: &str, value: &str) {
    let out_dir: std::path::PathBuf = std::env::var_os("OUT_DIR").unwrap().into();
    let filepath = out_dir.join(filename);
    std::fs::write(filepath, value).unwrap();
}

// End of file

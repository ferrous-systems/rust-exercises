// add this to Cargo.toml
// ``` toml
// [dependencies]
// rand = "0.7.3"
// ```

use std::{collections::HashMap, ops::Range, str};

use rand::prelude::SliceRandom as _;

fn main() {
    const PRINTABLE_ASCII: Range<u8> = 32..127;
    // TODO change this
    let plaintext = b"example text";

    let mut rng = rand::thread_rng();
    let mut from = PRINTABLE_ASCII.collect::<Vec<_>>();
    from.shuffle(&mut rng);
    let mut to = PRINTABLE_ASCII.collect::<Vec<_>>();
    to.shuffle(&mut rng);

    let mut dict = HashMap::new();
    for (&from, &to) in from.iter().zip(to.iter()) {
        dict.insert(from, to);
    }

    let secret = plaintext.iter().map(|byte| dict[byte]).collect::<Vec<_>>();
    println!("from: {:?}", from);
    println!("to: {:?}", to);
    println!("plaintext: {}", str::from_utf8(plaintext).expect("was not ASCII"));
    println!(
        "secret: {:?}",
        str::from_utf8(&secret).expect("was not ASCII")
    );
}

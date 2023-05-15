use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn main() -> Result<(), Error> {
    let mut file = File::open("src/data/content.txt")?;

    let mut content_string = String::new();
    file.read_to_string(&mut content_string)?;

    println!("{}", content_string);
    Ok(())
}

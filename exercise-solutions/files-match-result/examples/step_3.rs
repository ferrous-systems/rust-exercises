use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let file = File::open("./src/data/content.txt")?;

    let buf_reader = BufReader::new(file);

    let mut number = 0;

    for _line in buf_reader.lines() {
        number += 1;
    }

    println!("{}", number);

    Ok(())
}

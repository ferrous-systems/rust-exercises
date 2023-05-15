use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Error> {
    let open_result = File::open("./src/data/content.txt")?;

    let buf_reader = BufReader::new(file);

    let mut number = 0;

    for _line in buf_reader.lines() {
        number += 1;
    }

    println!("{}", number);
    
    Ok(())
}

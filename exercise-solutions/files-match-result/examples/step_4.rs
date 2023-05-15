use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let file = File::open("src/data/content.txt")?;

    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let line = line?;
        if !line.is_empty() {
            println!("{}", line)
        }
    }

    Ok(())
}

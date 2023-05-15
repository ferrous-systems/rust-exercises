use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use url::Url;

fn parse_line(line: String) -> Option<Url> {
    match Url::parse(&line) {
        Ok(u) => Some(u),
        Err(_e) => None,
    }
}

fn main() -> Result<(), Error> {
    let file = File::open("src/data/content.txt")?;

    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let line = line?;
        if let Some(valid_url) = parse_line(line) {
            println!("{}", valid_url);
        }
    }
    
    Ok(())
}

#[test]
fn correct_url() {
    assert!(parse_line(String::from("https://example.com")).is_some())
}

#[test]
fn no_url() {
    assert!(parse_line(String::from("abcdf")).is_none())
}

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let open_result = File::open("src/data/content.txt");

    let file = match open_result {
        Ok(file) => file,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };

    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        match line {
            Ok(content) => {
                if !content.is_empty() {
                    println!("{}", content)
                }
            }
            Err(e) => println!("Error reading line {}", e),
        }
    }
}

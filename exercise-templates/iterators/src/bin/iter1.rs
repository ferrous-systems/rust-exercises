#![allow(unused_imports)]
use std::io::BufReader;
use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    use crate::*;
    let f = File::open("../exercise-templates/iterators/numbers.txt")?;
    let mut reader = BufReader::new(f);

    // Write your iterator chain here 
    let sum_of_odd_numbers: i32 = todo!("use reader.lines() and Iterator methods");

    assert_eq!(sum_of_odd_numbers, 20);
    Ok(())
}


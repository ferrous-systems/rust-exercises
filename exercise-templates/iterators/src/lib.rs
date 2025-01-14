#![allow(unused_imports)]
use std::io::BufReader;
use std::fs::File;
use std::error::Error;

#[test]
fn iterator_test() -> Result<(), Box<dyn Error>> {
    use crate::*;
    let f = File::open("numbers.txt")?;
    let mut reader = BufReader::new(f);

    // Write your iterator chain here 
    let sum_of_odd_numbers: i32 = todo!("use reader.lines() and Iterator methods");

    assert_eq!(sum_of_odd_numbers, 20);
    Ok(())
}


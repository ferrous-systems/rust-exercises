#![allow(unused_imports)]
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Idiomatic solution
    let second_reader = BufReader::new(File::open("../exercise-solutions/iterators/numbers.txt")?);
    let nicer_sum: i32 = second_reader.lines()
        .filter_map(|line| line.ok())
        .filter_map(|s| s.parse().ok())
        .filter(|num| num % 2 != 0)
        .sum::<i32>();

    assert_eq!(nicer_sum, 31);

    Ok(())
}

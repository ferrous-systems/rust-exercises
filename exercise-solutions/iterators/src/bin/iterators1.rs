#![allow(unused_imports)]
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    use crate::*;
    let f = File::open("../exercise-solutions/iterators/numbers.txt")?;
    let reader = BufReader::new(f);

    // Write your iterator chain here
    let sum_of_odd_numbers: i32 = reader.lines()
        .map(|l| l.unwrap())              // peel off each line from the BufReader until you're done
        .map(|s| s.parse())               // try to parse the line as a number, yields a `Result`
        .filter(|s| s.is_ok())            // keep the lines that actually parsed (they're the `Ok` variant)
        .map(|l| l.unwrap())              // unwrap the succesful parses, which yield numbers
        .filter(|num| num % 2 != 0)       // keep the odd numbers
        .collect::<Vec<i32>>()            // collect the numbers into a vector
        .iter()                           // iterate over the vector
        .fold(0, |acc, elem| acc + elem); // fold over the vector and add the elements, yields an i32

    assert_eq!(sum_of_odd_numbers, 31);

    // Idiomatic solution
    let second_reader = BufReader::new(File::open("../exercise-solutions/iterators/numbers.txt")?);
    let nicer_sum: i32 = second_reader.lines()
        .filter_map(|line| line.ok())
        .filter_map(|s| s.parse().ok())   // map a .parse() and filter for the succesful parses
        .filter(|num| num % 2 != 0)
        .sum::<i32>();

    assert_eq!(nicer_sum, 31);

    Ok(())
}

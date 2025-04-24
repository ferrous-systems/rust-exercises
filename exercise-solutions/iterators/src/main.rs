use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("numbers.txt")?;
    let reader = BufReader::new(f);

    let sum_of_odd_numbers: i32 = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|s| s.parse().ok())
        .filter(|num| num % 2 != 0)
        .fold(0, |acc, elem| acc + elem);
        // Also works
        //.sum::<i32>();

    println!("Sum is {}", sum_of_odd_numbers);
    assert_eq!(sum_of_odd_numbers, 31);

    Ok(())
}

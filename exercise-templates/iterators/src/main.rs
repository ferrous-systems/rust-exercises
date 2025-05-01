use std::error::Error;
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("numbers.txt")?;
    let reader = BufReader::new(f);

    // Write your iterator chain here
    let sum_of_odd_numbers: i32 = todo!("use reader.lines() and Iterator methods");

    assert_eq!(sum_of_odd_numbers, 31);
    Ok(())
}

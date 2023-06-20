fn main() -> Result<(), std::io::Error> {
    let file_contents = std::fs::read_to_string("src/data/content.txt")?;
    println!("File opened and read");

    let mut number = 0;

    for _line in file_contents.lines() {
        number += 1;
    }

    println!("{}", number);

    Ok(())
}

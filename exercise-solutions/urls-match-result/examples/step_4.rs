fn main() -> Result<(), std::io::Error> {
    let file_contents = std::fs::read_to_string("src/data/content.txt")?;
    println!("File opened and read");

    for line in file_contents.lines() {
        if !line.is_empty() {
            println!("{}", line)
        }
    }

    Ok(())
}

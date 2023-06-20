fn main() -> Result<(), std::io::Error> {
    let _file_contents = std::fs::read_to_string("src/data/content.txt")?;
    println!("File opened and read");
    Ok(())
}

fn main() {
    let read_result = std::fs::read_to_string("src/data/content.txt");

    match read_result {
        Ok(_str) => println!("File opened and read"),
        Err(e) => panic!("Problem opening and reading the file: {:?}", e),
    };
}

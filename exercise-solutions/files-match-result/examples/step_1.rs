use std::fs::File;

fn main() {
    let open_result = File::open("src/data/content.txt");

    match open_result {
        Ok(_file) => println!("File opened"),
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };
}

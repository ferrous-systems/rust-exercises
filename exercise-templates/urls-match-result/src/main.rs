fn main() {
    let _file_contents = std::fs::read_to_string("src/lib/content.txt").unwrap();
    //                            ^^^^^^^^^^^^^^
    //                                  std::fs::read_to_string yields a Result<String, Error>,
    //                                  and a quick way to get to the value of type String is
    //                                  to use the unwrap() method on the Result<String, Error>.
}

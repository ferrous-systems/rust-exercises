use std::collections::HashMap;

fn main() {
    println!("-- Collections Map Exercise --");

    println!("Enter key-value pairs (key1:value1,key2:value2): ");

    let mut pairs_input = String::new();
    std::io::stdin()
        .read_line(&mut pairs_input)
        .expect("reading input string failed");

    let mut map = HashMap::new();
    for pair in pairs_input.trim().split(',') {
        let (key, value) = pair
            .split_once(':')
            .expect("expected 'key: value' pairs separated by commas");
        map.insert(key.trim(), value.trim());
    }
    println!("Parsed map: {:?}", map);
}

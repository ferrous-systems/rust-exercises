use std::collections::BTreeSet;

fn main() {
    println!("-- Collections Exercise --");

    println!("Please enter a sentence, the words will be sorted:");

    let mut words_input = String::new();
    std::io::stdin()
        .read_line(&mut words_input)
        .expect("reading input string failed");

    let mut words = Vec::new();
    for word in words_input.split_whitespace() {
        words.push(word);
    }
    // Alternative solution using collect:
    // let mut words: Vec<&str> = words_input.split_whitespace().collect();

    words.sort();
    // Alternatively: `words.sort_unstable`.
    println!("Sorted words (Vec): {:?}", words);

    let mut words_set = BTreeSet::new();
    for word in words_input.split_whitespace() {
        words_set.insert(word);
    }
    // Alternative solution using collect:
    // let words_set: BTreeSet<&str> = words_input.split_whitespace().collect();
    println!("Sorted words (BTreeSet): {:?}", words_set);
}

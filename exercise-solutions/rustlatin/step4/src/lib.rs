const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
// ^^^^^^^^^ The vowels are contained in an array, because the length never changes.
//           It's a global const because it will not be modified in any way and it's
//           small enough that the way that const variables are copied into each
//           usage location isn't a problem.

/// Performs a "rust-latin" conversion on the given string
fn rustlatin(sentence: &str) -> String {
    let mut collection_of_words = Vec::new();

    for word in sentence.split(' ') {
        let latinized_word = latinize(word);
        collection_of_words.push(latinized_word)
    }
    collection_of_words.join(" ")
}

/// adds prefix "sr" and suffix "rs" according to the rules
fn latinize(word: &str) -> String {
    let first_char = word.chars().next().unwrap();
    if VOWELS.contains(&first_char) {
        let mut result = "sr".to_string();
        result.push_str(word);
        result
    } else {
        let mut result = word.to_string();
        result.push_str("rs");
        result
    }
}

#[test]
fn test_latinizer() {
    assert_eq!(latinize("rust"), "rustrs");
    assert_eq!(latinize("helps"), "helpsrs");
    assert_eq!(latinize("you"), "yours");
    assert_eq!(latinize("avoid"), "sravoid");
}

#[test]
fn correct_translation() {
    // Why can we compare `&str` and `String` here?
    // https://doc.rust-lang.org/stable/std/string/struct.String.html#impl-PartialEq%3C%26%27a%20str%3E
    assert_eq!(
        "rustrs helpsrs yours sravoid sra lotrs srof srirritating bugsrs",
        rustlatin("rust helps you avoid a lot of irritating bugs")
    )
}

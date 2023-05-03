fn rustlatin(sentence: &str) -> Vec<String> {
    //                          ^^^^^^^
    // The correct return type needs to be added by you,
    // depending on what the vector's exact type is.
    let mut collection_of_words = Vec::new();

    for word in sentence.split(' ') {
        // Your implementation goes here:
        // Add the suffix "rs" to each word before pushing it to the vector
        let mut word = word.to_string();
        word.push_str("rs");
        collection_of_words.push(word);
    }
    collection_of_words
}

#[test]
fn concatenated() {
    assert_eq!(
        vec!["dors", "yours", "likers", "rustrs"],
        rustlatin("do you like rust")
    )
}

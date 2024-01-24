#![allow(unused)]

fn rustlatin(sentence: &str) -> Vec<String> {
    //                          ^^^^^^^
    // The correct return type needs to be added by you,
    // depending on what the vector's exact type is.

    let mut collection_of_words = Vec::new();
    //                         ^^^^^^^^^^^^
    // When you first open this file RA is not able to infer
    // the type of this vector. Once you do the implementation,
    // the type should appear here automatically.

    // Your implementation goes here:
    // Iterate over the sentence to split it into words.
    for word in sentence.split(' ') {
        collection_of_words.push(word.to_string())
    }
    // Push the words into the vector.
    // Correct the return type of the vector
    collection_of_words
}

#[test]
fn correct_splitting() {
    assert_eq!(
        vec!["This", "sentence", "needs", "to", "be", "split"],
        rustlatin("This sentence needs to be split")
    )
}

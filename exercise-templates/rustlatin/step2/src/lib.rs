#![allow(unused)]

fn rustlatin(sentence: &str) -> Vec<()> {
    //                          ^^^^^^^
    // The correct return type needs to be added by you,
    // depending on what the vector's exact type is.
    let mut collection_of_words = Vec::new();

    for word in sentence.split(' ') {
        // Your implementation goes here:
        // Add the suffix "rs" to each word before pushing it to the vector
        // Correct the return type of the function.
    }
    collection_of_words
}

// #[test]
// fn concatenated() {
//     assert_eq!(
//         vec!["dors", "yours", "likers", "rustrs"],
//         rustlatin("do you like rust")
//     )
// }

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
// ^^^^^^^^^ The vowels are contained in an array, because the length never changes.
//           It's a global const because it will not be modified in any way and it's
//           small enough that the way that const variables are copied into each
//           usage location isn't a problem.

/// Performs a "rust-latin" conversion on the given string
fn rustlatin(sentence: &str) -> String {
    let mut collection_of_words = Vec::new();

    for word in sentence.split(' ') {
        // Your implementation goes here...
        // delete this, and push the latinized words into the vector
        collection_of_words.push("")
    }
    collection_of_words.join(" ")
}

/// adds prefix "sr" and suffix "rs" according to the rules
fn latinize() {
    // You need to add the right arguments and return type, then implement
    // this function.
    unimplemented!();
}

#[test]
fn test_latinizer() {
    // Uncomment these test cases
    // assert_eq!(latinize("rust"), "rustrs");
    // assert_eq!(latinize("helps"), "helpsrs");
    // assert_eq!(latinize("you"), "yours");
    // assert_eq!(latinize("avoid"), "sravoid");
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

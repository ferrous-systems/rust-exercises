# Rustlatin

In this exercise we will implement a Rust-y, simpler variant of [Pig
Latin](https://en.wikipedia.org/wiki/Pig_Latin): Depending on if a word
starts with a vowel or not, either a suffix or a prefix is added to the
word

## Learning Goals

You will learn how to:

-   create a Rust library

-   split a `&str` at specified `char`

-   get single `char` out of a `&str`

-   iterate over a `&str`

-   define Globals

-   compare a value to the content of an array

-   use the Rust compiler’s type inference to your advantage

-   to concatenate `&str`

-   return the content of a `Vec<String>` as `String`.

## Prerequisites

You must be able to 
* define variables as mutable 
* use for loop 
* use an if/else construction 
* read Rust documentation 
* define a function with signature and return type 
* define arrays and vectors 
* distinguish between `String` and `&str`

## Tasks

For this exercise we define

- the Vowels of English alphabet → `['a', 'e', 'i', 'o', 'u']`

- a sentence is a collection of Unicode characters with words that are separated by a space character (`U+0020`)

✅ Implement a function that splits a sentence into its words, and adds a suffix or prefix to them according to the following rules:

- If the word begins with a vowel add prefix “sr” to the word.

- If the word does not begin with a vowel add suffix “`rs`” to the word.

The function returns a `String` containing the modified words.

In order to learn as much as possible we recommend following the step-by-step solution.

### Getting started

Find the exercise template in [`../../exercise-templates/rustlatin`](../../exercise-templates/rustlatin)

The folder contains each step as its own numbered project, containing a `lib.rs` file. Each `lib.rs` contains starter code and a test that needs to pass in order for the step to be considered complete.

Complete solutions are available [`../../exercise-solutions/rustlatin`](../../exercise-solutions/rustlatin)

## Knowledge

### Rust Analyzer

A part of this exercise is seeing type inference in action and to use it to help to determine the type the function is going to return. To make sure the file can be indexed by Rust Analyzer, make sure you open the relevant step by itself - e.g. `exercise-templates/rustlatin/step1`. You can close each step when complete and open the next one.

# Step-by-step-Solution

## Step 1: Splitting a sentence and pushing its words into a vector.

✅ Iterate over the sentence to split it into words. Use the white space as separator. This can be done with the [`.split()`](https://doc.rust-lang.org/std/primitive.str.html#method.split) method, where the separator character `' '` goes into the paranthesis. This method returns an iterator over substrings of the string slice. In Rust, iterators are lazy, that means just calling `.split()` on a `&str` doesn’t do anything by itself. It needs to be in combination with something that advances the iteration, such as a `for` loop, or a manual advancement such as the `.next()` method. These will yield the actual object you want to use. [Push](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push) each word into the vector `collection_of_words`. Add the correct return type to the function signature.

✅ Run the test to see if it passes.

<details>
  <summary>Solution</summary>

```rust
fn rustlatin(sentence: &str) -> Vec<String> {
    let mut collection_of_words = Vec::new();

    for word in sentence.split(' ') {
        collection_of_words.push(word.to_string())
    }
    collection_of_words
}


# fn main() {
#    assert_eq!(
#        vec!["This", "sentence", "needs", "to", "be", "split"],
#        rustlatin("This sentence needs to be split")
#    )
#}
```
</details>

## Step 2: Concatenating String types.

✅ After iterating over the sentence to split it into words, add the suffix `"rs"` to each word before pushing it to the vector. 

✅ To concatenate two `&str` the first needs to be turned into the owned type with `.to_owned()`. Then `String` and `&str` can be added using `+`. 

✅ Add the correct return type to the function signature. 

✅ Run the test to see if it passes.

<details>
  <summary>Solution</summary>

```rust
fn rustlatin(sentence: &str) -> Vec<String> {
    let mut collection_of_words = Vec::new();

    for word in sentence.split(' ') {
            collection_of_mod_words.push(word.to_owned() + "rs")

    };
    collection_of_words
}
```
</details>

## Step 3: Iterating over a word to return the first character.

✅ After iterating over the sentence to split it into words, add the first character of each word to the vector.

✅ Check the Rust documentation on the [primitive str Type](https://doc.rust-lang.org/std/primitive.str.html#) for a method that returns an iterator over the `chars` of a `&str`. The `char` type holds a Unicode Scalar Value that represents a single *character* (although just be aware the definition of *character* is complex when talking about emojis and other non-English text).

Since iterators don’t do anything by themselves, it needs to be advanced first, with the `.next()` method. This method returns an `Option(Self::Item)`, where `Self::Item` is the `char` in this case. You don’t need to handle it with pattern matching in this case, a simple `unwrap()` will do, as a `None` is not expected to happen.

✅ Add the correct return type to the function signature. Run the test to see if it passes.

<details>
  <summary>Solution</summary>

```rust
fn rustlatin(sentence: &str) -> Vec<char> {
    let mut collection_of_chars = Vec::new();

    for word in sentence.split(' ') {
        let first_char = word.chars().next().unwrap();
        collection_of_chars.push(first_char);
    };
    collection_of_chars
}
```
</details>

## Step 4: Putting everything together: Comparing values and returning the content of the vector as `String`.

✅ Add another function that checks if the first character of each word is a vowel. [contains()](https://doc.rust-lang.org/std/primitive.slice.html#method.contains) is the method to help you with this. It adds the prefix or suffix to the word according to the rules above.

Call the function in each iteration.

In `fn rustlatin` return the content of the vector as `String`. Run the tests to see if they pass.

<details>
  <summary>Solution</summary>

```rust
fn latinize(word: &str) -> String {
    let first_char_of_word = word.chars().next().unwrap();
    if VOWELS.contains(&first_char_of_word) {
        "sr".to_string() + word
    } else {
        word.to_string() + "rs"
    }
}
```
</details>

## Step 5 (optional)

If not already done, use functional techniques (i.e. methods on [iterators](https://doc.rust-lang.org/std/iter/trait.Iterator.html)) to write the same function. Test this new function as well.

<details>
  <summary>Solution</summary>

```rust
fn rustlatin_match(sentence: &str) -> String {
    // transform incoming words vector to rustlatined outgoing
    let new_words: Vec<_> = sentence
        .split(' ')
        .into_iter()
        .map(|word| {
            let first_char_of_word = word.chars().next().unwrap();
            if VOWELS.contains(&first_char_of_word) {
                "sr".to_string() + word
            } else {
                word.to_string() + "rs"
            }
        })
        .collect();

    new_words.join(" ")
}
```
</details>

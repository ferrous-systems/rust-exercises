# Green and Yellow Game

In this assignment we will implement the game "Green and Yellow". It’s like Wordle, but with numerical digits instead of letters. But for legal reasons it’s also entirely unlike Wordle, nor remotely similar to the 1970’s board-game "Mastermind".

## After completing this exercise you are able to

- Work with rust slices and vectors
- Accept input from stdin
- Iterate through arrays and slices
- Generate random numbers

## Prerequisites

For completing this exercise you need to have:

- basic Rust programming skills
- the Rust Syntax Cheat Sheet

## Task

1. Create a new binary crate called `green-yellow`
1. Copy all the test cases into into your `main.rs`
1. Define a function `fn calc_green_and_yellow(guess: &[u8; 4], secret: &[u8; 4]) -> String` that implements the following rules:
    - Return a string containing four Unicode characters
    - For every item in guess, if `guess[i] == secret[i]`, then position `i` in the output String should be a green block (`🟩`)
    - Then, for every item in guess, if `guess[i]` is in `secret` somewhere, and hasn't already been matched, then position `i` in the output String should be a yellow block (`🟨`)
    - If any of the guesses do not appear in the secret, then that position in the output String should be a grey block (`⬜`)
2. Ensure all the test cases pass!
3. Write a main function that implements the following:
    - Generate 4 random digits - our 'secret'
    - Go into a loop
    - Read a string from *Standard In* and trim the whitespace off it
    - Parse that string into a guess, containing four digits (give an error if the user makes a mistake)
    - Run the calculation routine above and print the coloured blocks
    - Exit if all the blocks are green
4. Play the game

If you need it, we have provided a [complete solution](../../exercise-solutions/green-yellow/src/bin/complete.rs) for this exercise.

Your test cases are:

```rust
#[test]
fn all_wrong() {
    assert_eq!(
        &calc_green_and_yellow(&[5, 6, 7, 8], &[1, 2, 3, 4]),
        "⬜⬜⬜⬜"
    );
}

#[test]
fn all_green() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 2, 3, 4], &[1, 2, 3, 4]),
        "🟩🟩🟩🟩"
    );
}

#[test]
fn one_wrong() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 2, 3, 5], &[1, 2, 3, 4]),
        "🟩🟩🟩⬜"
    );
}

#[test]
fn all_yellow() {
    assert_eq!(
        &calc_green_and_yellow(&[4, 3, 2, 1], &[1, 2, 3, 4]),
        "🟨🟨🟨🟨"
    );
}

#[test]
fn one_wrong_but_duplicate() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 2, 3, 1], &[1, 2, 3, 4]),
        "🟩🟩🟩⬜"
    );
}

#[test]
fn one_right_others_duplicate() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 1, 1, 1], &[1, 2, 3, 4]),
        "🟩⬜⬜⬜"
    );
}

#[test]
fn two_right_two_swapped() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 2, 2, 2], &[2, 2, 2, 1]),
        "🟨🟩🟩🟨"
    );
}

#[test]
fn two_wrong_two_swapped() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 3, 3, 2], &[2, 2, 2, 1]),
        "🟨⬜⬜🟨"
    );
}

#[test]
fn a_bit_of_everything() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 9, 4, 3], &[1, 2, 3, 4]),
        "🟩⬜🟨🟨"
    );
}
```

## Knowledge

### Generating Random Numbers

There are no random number generators in the standard library - you have to use the `rand` crate.

You will need to change `Cargo.toml` to depend on the `rand` crate - we suggest version `0.8`.

You need a random number generator (call `rand::thread_rng()`), and using that you can generate a number out of a given range with `gen_range`. See <https://docs.rs/rand> for more details.

### Reading from the Console

You need to grab a standard input handle with `std::io::stdin()`. This implments the `std::io::Write` trait, so you can call `read_to_string(&mut some_string)` and get a line of text into your `some_string: String` variable.

### Parsing Strings into Integers

Strings have a `parse()` method, which returns a `Result`, because of course the user may not have typed in a proper digit. The `parse()` function works out what you are trying to create based on context - so if you want a `u8`, try `let x: u8 = my_str.parse().unwrap()`. Or you can say `let x = my_str.parse::<u8>().unwrap()`. Of course, try and do something better than unwrap!

## Step-by-Step-Solution

In general, we also recommend to use the Rust documentation to figure out things you are missing to familiarize yourself with it. If you ever feel completely stuck or that you haven’t understood something, please hail the trainers quickly.

### Step 1: New Project

Create a new binary Cargo project, check the build and see if it runs.

<details>
  <summary>Solution</summary>

```shell
cargo new green-yellow
cd fizzbuzz 
cargo run
```

</details>

### Step 2: Generate some squares

Get `calc_green_and_yellow` to just generate grey blocks. We put them in an `Vec` first, as that's easier to index than a string.

Call the function from `main()` to avoid the warning about it being unused.

<details>
  <summary>Solution</summary>

```rust ignore
{{#include ../../exercise-solutions/green-yellow/src/bin/step2.rs:3:7}}
```

</details>

### Step 3: Check for green squares

You need to go through every pair of items in the input arrays and check if they are the same. If so, set the output square to be green.

<details>
  <summary>Solution</summary>

```rust ignore
{{#include ../../exercise-solutions/green-yellow/src/bin/step3.rs:3:13}}
```

</details>

### Step 4: Check for yellow squares

This gets a little more tricky.

We need to loop through every item in the guess array and compare it to every item in the secret array. But! We must make sure we ignore any values we already 'used up' when we produced the green squares.

Let's do this by copying the input, so we can make it mutable, and mark off any values used in the green-square-loop by setting them to zero.

<details>
  <summary>Solution</summary>

```rust ignore
{{#include ../../exercise-solutions/green-yellow/src/bin/step4.rs:3:25}}
```

</details>

### Step 5: Get some random numbers

Add `rand = "0.8"` to your Cargo.toml, and make a random number generator with `rand::thread_rng()` (Random Number Generator). You will also have to `use rand::Rng;` to bring the trait into scope.

Call `your_rng.gen_range()` in a loop.

<details>
  <summary>Solution</summary>

```rust ignore
{{#include ../../exercise-solutions/green-yellow/src/bin/step5.rs:29:38}}
```

</details>

### Step 6: Make the game loop

We a loop to handle each guess the user makes.

For each guess we need to read from Standard Input (using `std::io::stdin()` and its `read_line()`) method.

You will need to `trim` and then `split` the input, then `parse` each piece into a digit.

* If the digit doesn't parse, `continue` the loop.
* If the digit parses but it out of range, `continue` the loop.
* If you get the wrong number of digits, `continue` the loop.
* If the guess matches the secret, then break out of the loop and congratulate the winner.
* Otherwise run the guess through our calculation function and print the squares.

<details>
  <summary>Solution</summary>

```rust ignore
{{#include ../../exercise-solutions/green-yellow/src/bin/step6.rs:41:72}}
```

</details>

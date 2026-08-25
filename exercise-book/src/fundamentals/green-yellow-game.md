# Green and Yellow Game

In this assignment we will implement the game "Green and Yellow". It’s like Wordle, but with numerical digits instead of letters. But for legal reasons it’s also entirely unlike Wordle, and entirely unlike the 1970’s board-game "Mastermind".

* The computer picks some random digits
* You provide your guess for each digit
* The computer replies with sequence of grey, green and yellow blocks - one for each digit you guess
    * If you guessed a digit in the right place, you get a green block for that guess
    * If you guessed a digit in the wrong place, you get a yellow block for that guess
    * Otherwise you get a grey block and you know the digit you guessed is not in the output

Here are some examples:

### Example 1 - total failure

|        |   |   |   |   |
|--------|---|---|---|---|
| Secret | 1 | 2 | 3 | 4 |
| Guess  | 5 | 6 | 7 | 8 |
| Output | ⬜ | ⬜ | ⬜ | ⬜ |


### Example 2 - one right

|        |    |   |   |   |
|--------|----|---|---|---|
| Secret | 1  | 2 | 3 | 4 |
| Guess  | 1  | 5 | 6 | 7 |
| Output | 🟩 | ⬜ | ⬜ | ⬜ |

### Example 3 - right digit, wrong place

|        |   |   |    |   |
|--------|---|---|----|---|
| Secret | 1 | 2 | 3  | 4 |
| Guess  | 5 | 6 | 1  | 7 |
| Output | ⬜ | ⬜ | 🟨 | ⬜ |

### Example 4 - duplicate digits

Of course, it gets tricky when you have repeated digits! You cannot match a any particular secret digit twice, so:

|        |   |   |   |    |
|--------|---|---|---|----|
| Secret | 1 | 2 | 3 | 4  |
| Guess  | 5 | 6 | 4 | 4  |
| Output | ⬜ | ⬜ | ⬜ | 🟩 |

The third output is grey because although the guess of `4` is in the secret, you matched the only `4` in the secret with the fourth digit in the guess (and you got a green block for it). 

## After completing this exercise you will be able to

- Write simple functions
- Accept input from *Standard In*
- Iterate through arrays with for loops
- Generate random numbers

## Prerequisites

To complete this exercise you need to have:

- basic Rust programming skills
- a computer you can execute Rust code on, interactively

## Task

1. Create a new binary crate called `green-yellow`
1. Copy all the test cases into the `main.rs`
1. Define a constant `NUM_DIGITS: usize` with the value `4`
1. Define a function `fn calc_green_and_yellow(guess: &[u8; NUM_DIGITS], secret: &[u8; NUM_DIGITS]) -> [char; NUM_DIGITS]` that implements the following rules:
    - For each digit, if `guess[i] == secret[i]`, then the matching place in output should be a green block (`'🟩'`)
    - Then, for each digit, if `guess[i]` matches *any other digit* in `secret` (and that secret digit hasn't already been matched against something else) then the matching place in the output should be a yellow block (`'🟨'`)
    - Any unmatched digit in `guess` should get a grey block (`'⬜'`) as its output
    - **Note:** The output should only contain `'⬜'`, `'🟨'`, or `'🟩'` characters
1. Ensure all the test cases pass! (see below)
1. Write a `main` function that implements the following:
    - Generate 4 random digits - our 'secret'
    - Go into a loop
    - Read a string from *Standard In* and trim the whitespace off it
    - Parse that string digit by digit into a `[u8; NUM_DIGITS]` (and give an error if the user makes a mistake)
    - Run the calculation routine above and print the coloured blocks
    - Exit if all the blocks are green (or if `guess == secret`)
1. Play the game!

If you need it, we have provided a [complete solution](../../../exercise-solutions/green-yellow/src/bin/complete.rs) for this exercise.

Here are some test cases to check your algorith (scroll past them to get to the hints section, if you need a hint):

```rust
#[test]
fn all_wrong() {
    assert_eq!(
        calc_green_and_yellow(&[5, 6, 7, 8], &[1, 2, 3, 4]),
        ['⬜', '⬜', '⬜', '⬜']
    );
}

#[test]
fn all_green() {
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 3, 4], &[1, 2, 3, 4]),
        ['🟩', '🟩', '🟩', '🟩']
    );
}

#[test]
fn one_wrong() {
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 3, 5], &[1, 2, 3, 4]),
        ['🟩', '🟩', '🟩', '⬜']
    );
}

#[test]
fn all_yellow() {
    assert_eq!(
        calc_green_and_yellow(&[4, 3, 2, 1], &[1, 2, 3, 4]),
        ['🟨', '🟨', '🟨', '🟨']
    );
}

#[test]
fn one_wrong_but_duplicate() {
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 3, 1], &[1, 2, 3, 4]),
        ['🟩', '🟩', '🟩', '⬜']
    );
}

#[test]
fn one_right_others_duplicate() {
    assert_eq!(
        calc_green_and_yellow(&[1, 1, 1, 1], &[1, 2, 3, 4]),
        ['🟩', '⬜', '⬜', '⬜']
    );
}

#[test]
fn two_right_two_swapped() {
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 2, 2], &[2, 2, 2, 1]),
        ['🟨', '🟩', '🟩', '🟨']
    );
}

#[test]
fn two_wrong_two_swapped() {
    assert_eq!(
        calc_green_and_yellow(&[1, 3, 3, 2], &[2, 2, 2, 1]),
        ['🟨', '⬜', '⬜', '🟨']
    );
}

#[test]
fn a_bit_of_everything() {
    assert_eq!(
        calc_green_and_yellow(&[1, 9, 4, 3], &[1, 2, 3, 4]),
        ['🟩', '⬜', '🟨', '🟨']
    );
}

#[test]
fn two_in_guess_one_in_secret() {
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 3, 3], &[3, 9, 9, 9]),
        ['⬜', '⬜', '🟨', '⬜']
    );
}

#[test]
fn four_in_guess_one_in_secret() {
    assert_eq!(
        calc_green_and_yellow(&[1, 1, 1, 1], &[4, 3, 1, 2]),
        ['⬜', '⬜', '🟩', '⬜']
    );
}

#[test]
fn one_in_guess_two_in_secret() {
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 3, 4], &[3, 3, 9, 9]),
        ['⬜', '⬜', '🟨', '⬜']
    );
}
```

## Hints

### Generating Random Numbers

There are no random number generators in the standard library - you have to use the `rand` crate.

You will need to change `Cargo.toml` to depend on the `rand` crate - we suggest version `0.8`.

You need a random number generator (call `rand::thread_rng()`), and using that you can generate a number out of a given range with `gen_range`. See <https://docs.rs/rand> for more details.

### Reading from the Console

You need to grab a standard input handle with `std::io::stdin()`. This implements the `std::io::Read` trait, so you can call `read_to_string(&mut some_string)` and get a line of text into your `some_string: String` variable.

### Parsing Strings into Integers

Strings have a `parse()` method, which returns a `Result`, because of course the user may not have typed in a proper digit. The `parse()` function works out what you are trying to create based on context - so if you want a `u8`, try `let x: u8 = my_str.parse().unwrap()`. Or you can say `let x = my_str.parse::<u8>().unwrap()`. Of course, try to do something better than unwrap because it seems rude to crash the game if the player has a mistake with their input.

## Step-by-Step-Solution

If you aren't sure how to proceed, try this step by step guide.

If you ever feel completely stuck, or if you haven’t understood something specific, please hail the trainers quickly.

### Step 1: New Project

Create a new binary Cargo project, check it runs.

<details>
  <summary>Solution</summary>

```shell
cargo new green-yellow
cd green-yellow
cargo run
```

</details>

### Step 2: Generate some squares

Get `calc_green_and_yellow` to just generate an array of four grey blocks.

Call the function from `main()` to avoid the warning about it being unused.

<details>
  <summary>Solution</summary>

```rust ignore
{{#include ../../../exercise-solutions/green-yellow/src/bin/step2.rs:3:13}}
```

</details>

### Step 3: Check for green squares

You need to go through every pair of items in the input arrays and check if they are the same. If so, set the matching output square to be green.

<details>
  <summary>Solution</summary>

```rust ignore
{{#include ../../../exercise-solutions/green-yellow/src/bin/step3.rs:3:19}}
```

</details>

### Step 4: Check for yellow squares

This gets a little more tricky.

We need to loop through every item in the guess array and compare it to every item in the secret array. But! We must make sure we ignore any values we already 'used up' when we produced the green squares.

Let's make an array to record which secret digits have been used.

If you wanted, you could instead create a mutable copy of the `secret` array and set its digits to something invalid (like `0` or `255`) once they've been used.

<details>
  <summary>Solution</summary>

```rust ignore
{{#include ../../../exercise-solutions/green-yellow/src/bin/step4.rs:3:39}}
```

</details>

### Step 5: Get some random numbers

Add `rand = "0.8"` to your Cargo.toml, and make a random number generator (or 'RNG') with `rand::thread_rng()`. You will also have to `use rand::Rng;` to bring the trait into scope.

(A built-in [random number generator](https://github.com/rust-lang/rust/issues/130703) is proposed for the Standard Library but is still nightly only as of October 2024).

Call `your_rng.gen_range()` in a loop.

<details>
  <summary>Solution</summary>

```rust ignore
{{#include ../../../exercise-solutions/green-yellow/src/bin/step5.rs:43:52}}
```

</details>

### Step 6: Make the game loop

We need a loop to handle each guess the user makes and report the outcome of the guess.

For each guess we need to read from Standard Input (using `std::io::stdin()` and its `read_line()`) method.

You will need to `trim` and then `split` the input, then `parse` each piece into a digit.

* If the digit doesn't parse, print an error and `continue` the game loop.
* If the digit parses but it is out of range, print an error and `continue` the game loop.
* If you get the wrong number of digits, print an error and `continue` the game loop.
* Run the guess through our calculation function and print the squares.
* If the guess matches the secret, then break out of the loop and congratulate the winner.

<details>
  <summary>Solution</summary>

```rust ignore
{{#include ../../../exercise-solutions/green-yellow/src/bin/complete.rs:43:90}}
```

</details>

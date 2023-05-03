# Fizzbuzz

In this exercise, you will implement your first tiny program in rust: FizzBuzz. FizzBuzz is easy to implement, but allows for application of Rust patterns in a very clean fashion. If you have never written Rust before, use the [cheat sheet](/fizzbuzz-cheat-sheet.md/) for help on syntax.

## After completing this exercise you are able to

- write a simple Rust program
- create and return owned `String` s
- use conditionals
- format strings with and without printing them to the system console
- write a function with a parameter and return type.

## Prerequisites

For completing this exercise you need to have

- basic programming skills in other languages
- the Rust Syntax Cheat Sheet

## Task

- Create a new project called `fizzbuzz`
- Define a function `fn fizzbuzz` that implements the following rules:
  - If `i` is divisible by `3`, return the String "Fizz"
  - If `i` is divisible by `5`, return the String "Buzz"
  - If `i` is divisible by both `3` and `5`, return the String "FizzBuzz"
  - If neither of them is true return the number as a String

- Write a main function that implements the following:

  - Iterate from `1` to `100` inclusive.
  - On each iteration the integer is tested with `fn fizzbuzz`
  - print the returned value.

If you need it, we have provided a [complete solution](../../exercise-solutions/fizzbuzz/src/examples/fizzbuzz.rs) for this exercise.

## Knowledge

### Printing to console

The recommended way to print to the console in this exercise is `println!`. `println!` *always* needs a format string - it uses `{}` as a placeholder to mean **print the next argument**, like Python 3 or C#.

```rust
let s = String::from("Fizz");
println!("The value is s is {}. That's nice.", s);
```

### Creating Strings

The two recommended ways to get a `String` type for this exercise are:

```rust
// 1.
let string = String::from("Fizz");

let i = 4;
let string = i.to_string();

// 2. 
let string = format!("Buzz");

let i = 4;
format!("{}", i);
```

### Returning data

If you have issues returning data from multiple branches of your solution, liberally use `return`.

```rust
# fn returner() -> String {
    # let x = 10;
    if x % 5 == 0 {
    return String::from("Buzz");
    }
    String::from("Fizz")
# }

```

## Step-by-Step-Solution

In general, we also recommend to use the Rust documentation to figure out things you are missing to familiarize yourself with it. If you ever feel completely stuck or that you haven’t understood something, please hail the trainers quickly.

### Step 1: New Project

Create a new binary Cargo project, check the build and see if it runs.

<details>
  <summary>Solution</summary>

```shell
cargo new fizzbuzz 
cd fizzbuzz 
cargo run
```

</details>

### Step 2: Counting from 1 to 100 in `fn main()`

Print the numbers from 1 to 100 (inclusive) to console. Use a `for` loop.
Running this code should print the numbers from 1 to 100.

<details>
  <summary>Solution</summary>

```rust
fn main() {
    for i in 1..=100 {
        println!("{}", i);
    }
}
```

</details>

### Step 3: The function `fn fizzbuzz`

✅ Function Signature

Create the function with the name `fizzbuzz`. It takes an unsigned 32-bit integer as an argument and returns a `String` type.

<details>
  <summary>Solution</summary>

```rust
fn fizzbuzz(i: u32) -> String {
    unimplemented!()
}
```

</details>

✅ Function Body

Use if statements with math operators to implement the following rules:

- If `i` is divisible by `3`, return the String "Fizz"
- If `i` is divisible by `5`, return the String "Buzz"
- If `i` is divisible by both `3` and `5`, return the String "FizzBuzz"
- If neither of them is true return the number as a String

Running this code should still only print the numbers from 1 to 100.

<details>
  <summary>Solution</summary>

```rust
fn fizzbuzz(i: u32) -> String {
    if i % 3 == 0 && i % 5 == 0 {
        format!("FizzBuzz")
    } else if i % 3 == 0 {
        format!("Fizz")
    } else if i % 5 == 0 {
        format!("Buzz")
    } else {
        format!("{}", i)
    }
}
```

</details>

### Step 4: Call the function

Add the function call to `fn fizzbuzz()` to the formatted string in the `println!()` statement. 

Running this code should print numbers, interlaced with `Fizz`, `Buzz` and `FizzBuzz` according to the rules mentioned above.

<details>
  <summary>Solution</summary>

```rust
# fn fizzbuzz(i: u32) -> String {
#     if i % 3 == 0 && i % 5 == 0 {
#         format!("FizzBuzz")
#     } else if i % 3 == 0 {
#         format!("Fizz")
#     } else if i % 5 == 0 {
#         format!("Buzz")
#     } else {
#         format!("{}", i)
#     }
# }

fn main() {
    for i in 1..=100 {
        println!("{}", fizzbuzz(i));
    }
}
```

</details>

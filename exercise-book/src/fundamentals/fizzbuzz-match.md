# Fizzbuzz with `match`

In this exercise you will modify your previously written fizzbuzz to use `match` statements instead of `if` statements.

## After completing this exercise you are able to

- use `match` statements
- define a tuple

## Prerequisites

For completing this exercise you need to have

- a working fizzbuzz

## Task

Rewrite the body of `fn fizzbuzz()` so the different cases are not distinguished with `if` statements, but with pattern matching of a tuple containing the remainders.

If you need it, we have provided a [complete solution](../../../exercise-solutions/fizzbuzz/examples/fizzbuzz_match.rs) for this exercise.

## Knowledge

### Tuple

A tuple is a collection of values of different types. Tuples are constructed using parentheses (), and each tuple itself is a value with type signature (T1, T2, ...), where T1, T2 are the types of its members. Functions can use tuples to return multiple values, as tuples can hold any number of values, including the `_` placeholder

```rust
// A tuple with a bunch of different types.
let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);
```

## Step-by-Step-Solution

We assume you have deleted the entire function body of `fn fizzbuzz()` before you get started.

### Step 1: The Tuple

Define a tuple that consists of the remainder of the integer `i` divided by 3 and the integer `i` divided by 5.

<details>
  <summary>Solution</summary>

```rust
# let i = 10_u32;
let multi_check_tuple = (i.is_multiple_of(3), i.is_multiple_of(5));
```

</details>

### Step 2: Add the `match` statement with its arms

The the for us relevant patterns of the tuple that we match for are a combination of `0` and the placeholder `_` (underscore). `_` stands for any value. Think about what combinations of `0` and `_` represent which rules. Add the `match` arms accordingly.

<details>
  <summary>Solution</summary>

```rust
# fn fizzbuzz(i: u32) -> String {
    # let multi_check_tuple = (i.is_multiple_of(3), i.is_multiple_of(5));
    match multi_check_tuple {
        (true, true) => format!("FizzBuzz"),
        (true, false) => format!("Fizz"),
        (false, true) => format!("Buzz"),
        (false, false) => format!("{}", i),
    }
# }
```

</details>

### Step 3: Shorten the code by matching on the tuple directly

<details>
  <summary>Solution</summary>

```rust
# fn fizzbuzz(i: u32) -> String {
    match (i.is_multiple_of(3), i.is_multiple_of(5)) {
        (true, true) => format!("FizzBuzz"),
        (true, false) => format!("Fizz"),
        (false, true) => format!("Buzz"),
        (false, false) => format!("{}", i),
    }
# }
```

</details>

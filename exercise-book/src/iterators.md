# Iterators

In this exercise, you will learn to manipulate and chain iterators. Iterators are a functional way to write loops and control flow logic.

## After completing this exercise you are able to

- write a Rust iterator
- use closures in iterator chains
- collect a result to different containers
- iterate over slices and strings
- add and use the `itertools` library methods
- calculate a number of characteristics of the string "҈B҈҈E҈ ҈N҈҈O҈҈T҈ ҈A҈҈F҈҈R҈҈A҈҈I҈҈D҈"
- check if the following parentheses are balanced "(()()(()()(()(((())))()(()())()())))((())())"

## Prerequisites

For completing this exercise you need to have

- knowledge of control flow
- how to write basic functions
- basic types

## Task 1

- Add the odd numbers in the following string using an iterator chain

```text
//ignore everything that is not a number
1
2
3
4
five
6
7
∞
9
X
```

- Take the template in [exercise-templates/iterators](../../exercise-templates/iterators/) as a starting point.
- Replace the first `todo!` item with [reader.lines()]() and continue "chaining" the iterators until you've calculated the desired result.
- Run the code with `cargo run --bin iter1` when inside the `exercise-templates` directory

If you need it, we have provided a [complete solution](../../exercise-solutions/iterators/src/bin/iter1.rs) for this exercise.

## Knowledge

### Iterators and iterator chains

Iterators are a way to write for loops in a functional style. The main idea is to take away the error prone indexing and control flow by giving them a name that you and others can understand. For example, to double every number given by a vector, you could write a for loop:

```rust
let v = [10, 20, 30];
let mut xs = [0, 0, 0];

for idx in 0..=v.len() {
  xs[idx] = 2 * v[idx];
}
```

In this case, the name we give to the logic of `2 * v[idx]` and juggling the index over the entire collection is a [map](). An idiomatic Rustacean would write something similar to the following (period indented) code.

```rust
let v = [10, 20, 30];
let xs: Vec<_> = v.iter()
                  .map(|elem| elem * 2)
                  .collect();
```

This doesn't look like much of a win in terms of brevity, but it comes with a couple of benefits:

- Changing the underlying logic is more robust
- Less indexing operations means you will fight the borrow checker less
- You can parallelize your code with minimal changes using [rayon]()

The first point is not in vain - the original snippet has a bug in the upper bound, since `0..=v.len()` is inclusive!

Think of iterators as lazy functions - they only carry out computation when called with a `.collect()` or similar, not the `.map()` itself.

### Turbo fish syntax `::<>`

Iterators sometimes struggle to figure out the types of all intermediate steps and need assistance.

We can write

```rust
let xs = v.iter()
          .map(|elem| elem * 2)
          .collect::<Vec<i32>>();
```

instead to avoid having a `xs: Vec<_> = ...` that may need back and forth editing as we change the iterator. This `::<SomeType>` syntax is called the [turbo fish](), and change the entirety of the iterator: `.collect::<HashSet<i32>>()`.

### Dereferences

Rust will give you feedback on when you need to add an extra dereference (`*`) by telling you about expected input and actual types, and you'll need to write something like `.map(|elem| *elem * 2)` to correct your code. A tell tale sign of this is that the expected types and the actual type differ by the number of `&`'s present.

Remember you can select and hover over each expression and rust-analyzer will display its type if you want a more detailed look inside.

## Destructuring in closures

Not all iterator chains operate on a single iterable at a time. This may mean joining several iterators and processing them together by destructuring a tuple:

```rust
let x = [10, 20, 30];
let y = [1, 2, 3];
let z = x.iter().zip(y.iter())
         .map(|(a, b)| a * b)
         .sum::<i32>();
```

where the `.map(|(a, b)| a + b)` is taking iterating over `[(10, 1), (20, 2), (30, 3)]` and calling the left argument `a` and the right argument `b`, in each iteration.

This code is basically equivalent to having written the named version of the closure and feeding it to `.map()`:

```rust
fn multiplier(x: i32, y: i32) -> i32 {
  x * y
}

let x = [10, 20, 30];
let y = [1, 2, 3];
let z = x.iter().zip(y.iter())
         .map(multiplier)
         .sum::<i32>();
```

## Step-by-Step-Solution

In general, we also recommend to use the Rust documentation to figure out things you are missing to familearize yourself with it, and the methods in the [Iterator](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html) page of the standard library.

If you ever feel completely stuck or that you haven’t understood something, please hail the trainers quickly.

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

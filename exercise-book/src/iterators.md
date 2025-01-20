# Iterators

In this exercise, you will learn to manipulate and chain iterators. Iterators are a functional way to write loops and control flow logic.

## After completing this exercise you are able to

- chain Rust iterator adapters
- use closures in iterator chains
- collect a result to different containers

## Prerequisites

For completing this exercise you need to have

- knowledge of control flow
- how to write basic functions
- know basic Rust types

## Task

- Calculate the sum of all odd numbers in the following string using an iterator chain

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
- Run the code with `cargo run --bin iterators1` when inside the `exercise-templates` directory

If you need it, we have provided a [complete solution](../../exercise-solutions/iterators/src/bin/iterators1.rs) for this exercise.

## Knowledge

### Iterators and iterator chains

Iterators are a way to write for loops in a functional style. The main idea is to take away the error prone indexing and control flow by giving them a name that you and others can understand and compose safely.

For example, to double every number given by a vector, you could write a for loop:

```rust
let v = [10, 20, 30];
let mut xs = [0, 0, 0];

for idx in 0..=v.len() {
  xs[idx] = 2 * v[idx];
}
```

In this case, the name we give to the procedure `2 * v[idx]` and juggling the index over the entire collection is a [map](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.map). An idiomatic Rustacean would write something similar to the following (period indented) code.

```rust
let v = [10, 20, 30];
let xs: Vec<_> = v.iter()
                  .map(|elem| elem * 2)
                  .collect();
```

No win for brevity, but it has several benefits:

- Changing the underlying logic is more robust
- Less indexing operations means you will fight the borrow checker less in the long run
- You can parallelize your code with minimal changes using [rayon](https://crates.io/crates/rayon).

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

instead to avoid having a `xs: Vec<_> = ...`. This `::<SomeType>` syntax is called the [turbo fish operator](https://doc.rust-lang.org/book/appendix-02-operators.html?highlight=turbo%20fish#non-operator-symbols), and it disambiguates calling the same method with different output types, like `.collect::<HashSet<i32>>()` and `.collect::<Vec<i32>>()` (try it!)

### Dereferences

Rust will often admonish you to add an extra dereference (`*`) by comparing the expected input and actual types, and you'll need to write something like `.map(|elem| *elem * 2)` to correct your code. A tell tale sign of this is that the expected types and the actual type differ by the number of `&`'s present.

Remember you can select and hover over each expression and rust-analyzer will display its type if you want a more detailed look inside.

## Destructuring in closures

Not all iterator chains operate on a single iterable at a time. This may mean joining several iterators and processing them together by destructuring a tuple when declaring the closure:

```rust
let x = [10, 20, 30];
let y = [1, 2, 3];
let z = x.iter().zip(y.iter())
         .map(|(a, b)| a * b)
         .sum::<i32>();
```

where the `.map(|(a, b)| a + b)` is iterating over `[(10, 1), (20, 2), (30, 3)]` and calling the left argument `a` and the right argument `b`, in each iteration.

## Step-by-Step-Solution

⚠️ NOTICE! ⚠️

When starting out with iterators, it's very easy to be "led astray" by doing what is locally useful as suggested by the compiler.

Concretely, our first solution will feel like a slog because we'll deal with a lot of `Option` and `Result` wrapping and unwrapping that other languages wouldn't make explicit.

A second more idiomatic solution will emerge in `Step 6` once we learn a few key idioms from the standard library.

You, unfortunately, relive similar experiences when learning Rust without knowing the right tools from the standard library to handle errors elegantly.

🧘 END OF NOTICE 🧘

We highly recommend that you consider turning off `inlay hints` in your `rust-analyzer` settings to `offUnlessPressed`, as they can get very noisy very quickly. You can do this by searching for `inlay hints` and choosing the right option in `Settings > Editor > Inlay Hints > Enabled`.

In general, we also recommend using the Rust documentation to get unstuck. In particular, look for the examples in the [Iterator](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html) page of the standard library for this exercise.

If you ever feel completely stuck or that you haven’t understood something, please hail the trainers quickly.

### Step 1: New Project

Create a new binary Cargo project, check the build and see if it runs.

Alternatively, use the [exercise-templates/iterators](../../exercise-templates/iterators/) template to get started.
<details>
  <summary>Solution</summary>

```shell
cargo new iterators
cd iterators 
cargo run

# if in exercise-book/exercise-templates/iterators
cargo run --bin iterators1
```

</details>

### Step 2: Read the string data

Read the string data from a file placed in `iterators/numbers.txt`.
Use the `reader.lines()` method to get rid of the newline characters.
Collect it into a string with `.collect::<String>()` and print it to verify you're ingesting it correctly. It should have no newline characters since `lines()` trimmed them off.

<details>
  <summary>Solution</summary>

```rust
#![allow(unused_imports)]
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    use crate::*;
    let f = File::open("../exercise-templates/iterators/numbers.txt")?;
    let reader = BufReader::new(f);

    let file_lines = reader.lines()
      .map(|l| l.unwrap())
      .collect::<String>();
    println!("{:?}", file_lines);

    Ok(())
}
```

</details>

### Step 3: Filter for the numeric strings

We'll collect into a `Vec<String>`s with [.parse()](https://doc.rust-lang.org/stable/std/primitive.str.html#method.parse) to show this intermediate step.

Note that you may or may not need type annotations on `.parse()` depending on if you add them on the binding or not - that is, `let numeric_lines: Vec<i32> = ...` will give Rust type information to deduce the iterator's type correctly.

<details>
  <summary>Solution</summary>

```rust
#![allow(unused_imports)]
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    use crate::*;
    let f = File::open("../exercise-templates/iterators/numbers.txt")?;
    let reader = BufReader::new(f);

    let numeric_lines = reader.lines()
      .map(|l| l.unwrap())
      .map(|s| s.parse::<i32>())
      .filter(|s| s.is_ok())
      .map(|l| l.unwrap().to_string())
      .collect::<Vec<String>>();
    println!("{:?}", numeric_lines);

    Ok(())
}
```

</details>

### Step 4: Keep the odd numbers

Use a [.filter()](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.filter) with an appropriate closure.

<details>
  <summary>Solution</summary>

```rust
#![allow(unused_imports)]
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    use crate::*;
    let f = File::open("../exercise-templates/iterators/numbers.txt")?;
    let reader = BufReader::new(f);

    let odd_numbers = reader.lines()
      .map(|l| l.unwrap())
      .map(|s| s.parse())
      .filter(|s| s.is_ok())
      .map(|l| l.unwrap())
      .filter(|num| num % 2 != 0)
      .collect::<Vec<i32>>();

    println!("{:?}", odd_numbers);

    Ok(())
}
```

</details>

### Step 5: Add the odd numbers

Take the odd numbers, `.collect()` into a vector, and add them using a [.fold()](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.fold).

You will probably reach for a `.sum::<i32>()`, but `.fold()`s are common enough in idiomatic Rust that we wanted to showcase one here.

<details>
  <summary>Solution</summary>

```rust
#![allow(unused_imports)]
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    use crate::*;
    let f = File::open("../exercise-templates/iterators/numbers.txt")?;
    let reader = BufReader::new(f);

    let result = reader.lines()
      .map(|l| l.unwrap())
      .map(|s| s.parse())
      .filter(|s| s.is_ok())
      .map(|l| l.unwrap())
      .filter(|num| num % 2 != 0)
      .collect::<Vec<i32>>()
      .iter()
      .fold(0, |acc, elem| acc + elem);
    // Also works
    //.sum::<i32>();

    println!("{:?}", result);

    Ok(())
}
```

</details>

### Step 6: Idiomatic Rust

That first solution can be a *slog*. 

Try writing a shorter solution using a [.filter_map()](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.filter_map).

<details>
  <summary>Solution</summary>

```rust
#![allow(unused_imports)]
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    use crate::*;
    let f = File::open("../exercise-templates/iterators/numbers.txt")?;
    let reader = BufReader::new(f);

    let result = reader.lines()
      .map(|l| l.unwrap())
      .filter_map(|s| s.parse().ok())
      .filter(|num| num % 2 != 0)
      .sum::<i32>();

    println!("{:?}", result);

    Ok(())
}
```

</details>

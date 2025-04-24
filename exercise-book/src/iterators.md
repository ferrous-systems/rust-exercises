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

Calculate the sum of all odd numbers in the following string using an iterator chain:

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
11
```

We have a [template project](../../exercise-template/iterators) for this exercise. You can replace the `todo!` item in the template with [reader.lines()](https://doc.rust-lang.org/stable/std/io/trait.BufRead.html#method.lines) and continue "chaining" the iterators until you've calculated the desired result. Note that the template will only be able to find `numbers.txt` if you run `cargo run` from the `exercise-templates/iterators` directory. Running the binary from elsewhere in the workspace will give a *File not found* error.

If you need it, we have provided a [complete solution](../../exercise-solutions/iterators/src/main.rs) for this exercise.

## Knowledge

### Iterators and iterator chains

Iterators are a way to chain function calls instead of writing elaborate for loops.

This lets us have a type safe way of composing control flow together by calling the right functions.

For example, to double every number given by a vector, you could write a for loop:

```rust [], ignore
let v = [10, 20, 30];
let mut xs = [0, 0, 0];

for idx in 0..=v.len() {
  xs[idx] = 2 * v[idx];
}
```

In this case, the idea of running a procedure like `2 * v[idx]` whilst indexing over the entire collection is called a [map](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.map). Using iterator chains you could instead write something like:

```rust [], ignore
let v = [10, 20, 30];
let xs: Vec<_> = v
  .iter()
  .map(|elem| elem * 2)
  .collect();
```

No win for brevity, but it has several benefits:

- Changing the underlying logic is more robust
- Less indexing operations means you will fight the borrow checker less in the long run
- You can parallelize your code with minimal changes using [rayon](https://crates.io/crates/rayon).

The first point is not in vain - the original snippet has a bug in the upper bound, since `0..=v.len()` is inclusive! It should have been `0..v.len()`.

Finally, don't forget that iterators are lazy functions - they only carry out computation when a *consuming adapter* like `.collect()` is called, not when the `.map()` is added to the chain.

### Iterator chains workflow advice

Start every iterator call on a new line, so that you can see closure arguments and type hints for the iterator at the end of the line clearly.

When in doubt, write `.map(|x| x)` first to see what item types you get and decide on what iterator methods to use and what to do inside a closure based on that.

### Turbofish syntax `::<>`

Iterators sometimes struggle to figure out the types of all intermediate steps and need assistance.

```rust [], ignore
let numbers: Vec<_> = ["1", "2", "3"]
    .iter()
    .map(|s| s.parse::<i32>().unwrap())
    // a turbofish in the `parse` call above
    // helps a compiler determine the type of `n` below
    .map(|n| n + 1)
    .collect();
```

This `::<SomeType>` syntax is called the [turbofish operator](https://doc.rust-lang.org/book/appendix-02-operators.html?highlight=turbofish#non-operator-symbols), and it disambiguates calling the same method but getting back different return types, like `.parse::<i32>()` and `.parse::<f64>()` (try it!)

### Dealing with `.unwrap()`s in iterator chains

Intermediate steps in iterator chains often produce `Result` or `Option`.

You may be tempted to use `unwrap / expect` to get the inner values. However, there are usually better ways that don't require a potential panic.

Concretely, the following snippet:

```rust [], ignore
    let numbers: Vec<_> = ["1", "2", "3"]
        .iter()
        .map(|s| s.parse::<i32>())
        .filter(|r| r.is_ok())
        .map(|r| r.expect("all `Result`s are Ok here"))
        .collect();
```

can be replaced with a judicious use of [.filter_map()](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.filter_map):

```rust [], ignore
    let numbers: Vec<_> = ["1", "2", "3"]
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
```

You will relive similar experiences when learning Rust without knowing the right tools from the standard library that let you convert `Result` into what you actually need.

We make a special emphasis on avoiding "`.unwrap()` now, refactor later" because later usually never comes.

### Dereferences

Rust will often admonish you to add an extra dereference (`*`) by comparing the expected input and actual types, and you'll need to write something like `.map(|elem| *elem * 2)` to correct your code. A tell tale sign of this is that the expected types and the actual type differ by the number of `&`'s present.

Remember you can select and hover over each expression and rust-analyzer will display its type if you want a more detailed look inside.

### Destructuring in closures

Some iterator chains involve `Item` being a tuple. If so, it may be useful to destructure the tuple when writing the closure:

```rust [], ignore
let x = [10, 20, 30];
let y = [1, 2, 3];
let z = x
  .iter()
  .zip(y.iter())
  .map(|(a, b)| a * b)
  .sum::<i32>();
```

Here, the `.map(|(a, b)| a + b)` is iterating over `[(10, 1), (20, 2), (30, 3)]` and calling the left argument `a` and the right argument `b`, in each iteration.

## Step-by-Step-Solution

In general, we also recommend using the Rust documentation to get unstuck. In particular, look for the examples in the [Iterator](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html) page of the standard library for this exercise.

If you ever feel completely stuck or that you haven’t understood something, please hail the trainers quickly.

### Step 1: New Project

Copy or recreate the [exercise-templates/iterators](../../exercise-templates/iterators/) template to get started.

### Step 2: Read the string data

Read the contents of `iterators/numbers.txt` line by line, and collect it all into one big `String`. Note that the `lines()` iterator gives us `Result<String, std::io::Error>` so let's only keep the lines that we were able to succesfully read from disk.

<details>
  <summary>Solution</summary>

```rust no_run
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    use crate::*;
    let f = File::open("numbers.txt")?;
    let reader = BufReader::new(f);

    let file_lines = reader.lines()
        .filter_map(|line| line.ok())
        .collect::<String>();

    println!("{:?}", file_lines);

    Ok(())
}
```

</details>

### Step 3: Skip the non-numeric lines

Now let's check that each line is a a valid number, using  [.parse()](https://doc.rust-lang.org/stable/std/primitive.str.html#method.parse). We'll be collecting everything into a `Vec<i32>`.

Note that you may or may not need type annotations on `.parse()` depending on if you add them on the binding or not - that is, `let numeric_lines: Vec<i32> = ...` will give Rust type information to deduce the iterator's type correctly.

<details>
  <summary>Solution</summary>

If the use of `filter_map` here is unfamiliar, go back and reread the ``Dealing with .unwrap()s in iterator chains`` section.

```rust no_run
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    use crate::*;
    let f = File::open("numbers.txt")?;
    let reader = BufReader::new(f);

    let numeric_lines: Vec<i32> = reader.lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<i32>().ok())
        .collect::<Vec<i32>>();
    println!("{:?}", numeric_lines);

    Ok(())
}
```

</details>

### Step 4: Keep the odd numbers

Use a [.filter()](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.filter) with an appropriate closure to keep only the odd numbers.

<details>
  <summary>Solution</summary>

```rust no_run
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    use crate::*;
    let f = File::open("numbers.txt")?;
    let reader = BufReader::new(f);

    let odd_numbers = reader.lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<i32>().ok())
        .filter(|num| num % 2 != 0)
        .collect::<Vec<i32>>();

    println!("{:?}", odd_numbers);

    Ok(())
}
```

</details>

### Step 5: Add the odd numbers

Take the odd numbers, and add them using a [.fold()](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.fold).

You will probably reach for a `.sum::<i32>()`, but `.fold()`s are common enough in idiomatic Rust that we wanted to showcase one here.

<details>
  <summary>Solution</summary>

```rust no_run
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    use crate::*;
    let f = File::open("numbers.txt")?;
    let reader = BufReader::new(f);

    let result = reader.lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<i32>().ok())
        .filter(|num| num % 2 != 0)
        .fold(0, |acc, elem| acc + elem);
        // Also works
        //.sum::<i32>();

    println!("{:?}", result);

    Ok(())
}
```

</details>

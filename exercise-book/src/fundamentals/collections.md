# Collections

In this exercise, you will learn how to work with Rust standard collections.
The documentation of the [`std::collections`](https://doc.rust-lang.org/std/collections/) is
excellent and provides an overview of the most important containers which are also included
in the standard library.

You will learn to:

* Read user input from the terminal.
* Use the Rust `std::vec::Vec` dynamic array container.
* Sort sequences.
* Use the Rust `std::collections::BTreeSet` sorted set container.
* Use the `std::collections::HashMap` map container.

You can find the solutions [here](../../../exercise-solutions/collections/).

## Task

1. Create a new binary crate called `collections`
1. Read in a string of space separated words into a vector. Then sort the vector
   and print the sorted list.
1. Perform the same task as in task 2, but use the `std::collections::BTreeSet`
   container instead which keeps the elements in sorted order. Print the list again.
1. Create a new binary inside the binary crate by creating a `bin` folder inside the `src` folder
   and creating a new `map.rs` file inside it.
1. Read in a string with the syntax `key1:value1,key2:value2` and store the
   key-value pairs in a hash map. A nice solution also deals with extra whitespace,
   e.g. `key1: value1, key2: value2`.

## Hints

These hints can help you with completing individual steps.

### Reading input from the console

The [`std::io`](https://doc.rust-lang.org/std/io/index.html) module contains everything
required to read from the console or write to the console. The operating
system models those using pre-defined [standard streams](https://en.wikipedia.org/wiki/Standard_streams).
These stdin, stdout and stderr are exposed by the [`std::io::stdin`](https://doc.rust-lang.org/std/io/fn.stdin.html), [`std::io::stdout`](https://doc.rust-lang.org/std/io/fn.stdout.html)
and [`std::io::stderr`](https://doc.rust-lang.org/std/io/fn.stderr.html) functions.

The [`std::io::Stdin`](https://doc.rust-lang.org/std/io/struct.Stdin.html) handle implements the
[`std::io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html) trait and also exposes the
[`read_line`](https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line)
method to help with this task. This function expects a mutable reference to a `std::string::String`,
so you need to create an empty one before calling it.

<details>

Code to read a line from the console:

```rust
    let mut input_string = std::string::String::new();
    std::io::stdin().read_line(&mut input_string).expect("reading input string failed");
```

</details>

### Word extraction

Rust provides a lot of useful APIs for string manipulation and extraction. You might
find the following APIs useful:

<details>

- [`split`](https://doc.rust-lang.org/std/primitive.str.html#method.split) and the variants
  [`split_once`](https://doc.rust-lang.org/std/primitive.str.html#method.split_once) and
  [`split_whitespace`](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace)
- [`trim`](https://doc.rust-lang.org/std/primitive.str.html#method.trim)

</details>

### Sorting a vector

The [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) data structure has a lot of useful
methods, including sorters.

<details>

You can use both the [`sort`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort) and the
[`sort_unstable`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_unstable) method
which sort in-place and do not require allocation.

</details>

## Quiz

After you have worked through this exercise, you can work through this quiz for a deeper
understanding:

**Q1**:

How does the `sort` method or the `BTreeSet` know how to order elements? Why did it just magically
know how to sort our strings/words?

Hint:

<details>

Have a look at the [sort API](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort) or the
[BTreeSet docs](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html).

</details>

Answer:

<details>

**A**:

All APIs or data structures which do sorting rely on a `core::cmp::Ord` implementation of
whatever type you are sorting. This trait is already implemented by commonly used standard
library types like [`core::str::str`](https://doc.rust-lang.org/std/primitive.str.html#impl-Ord-for-str)
and [`std::string::String`](https://doc.rust-lang.org/std/string/struct.String.html#impl-Ord-for-String).

</details>

**Q2a**:

If you have type hints enabled, you might notice that we are storing string references (`&str`)
inside our collections. The same is visible from the explicit type annotation in the
`collect`-based alternative solution.

Could you pass a collection containing string references to another thread? If yes or no,
why?

Hint:

<details>

Remember that a spawned thread can live longer than its enclosing scope. Is this problematic
when passing something with references to the thread?

</details>

Answer:

<details>

**A**:

No, this does not work. The closure argument for spawning a thread has a `'static` bound,
which is not fulfilled by `&str`. A thread can outlive its calling scope, so the `'static`
bound on the closure ensures that anything moved into the thread outlives the thread.

</details>

**Q2b**:

What would you need to do if you want to do this?

Hint:

<details>

Lifetimes only apply to mutable and shared references. You do not need to deal with them
when using owned types. `&str` has an owned variant.

</details>

Answer:

<details>

**A**:

Storing an owned type like `String` ensures we fulfill the lifetime bound. We can do this
by using `String::from` or using an explicit type annotation like `Vec<String>` in combination
with `.into()`.

</details>

**Q2c**:

What is the advantage of using `&str` over `String`?

Hint:

<details>

Check the [documentation of String](https://doc.rust-lang.org/std/string/struct.String.html).
How/Where is the string stored in memory?

</details>

Answer:

<details>

**A**:

String allocates on the heap and creates a copy of the word that was read in.
It is more expensive than a simple `&str` reference.

</details>


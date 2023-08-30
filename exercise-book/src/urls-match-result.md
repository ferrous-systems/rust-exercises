# URLs, Match and Result

In this exercise you will complete a number of mini exercises to learn about
Error Handling. The final result will be a url parser that reads lines from a
text file and can distinguish the content between URLs and non-urls.

## In this exercise, you will learn how to

- handle occurring `Result`-types with `match` for basic error
    handling.

- when to use the `.unwrap()` method.

- propagate an error with the `?` operator

- return the `Option`-type.

- do some elementary file processing (opening, reading to buffer,
    counting, reading line by line).

- navigate the Rust `stdlib` documentation

- add external dependencies to your project

## Task

Find the exercise template here
[`../../exercise-templates/urls-match-result`](../../exercise-templates/urls-match-result)

Find the solution to the exercise here
[`../../exercise-solutions/urls-match-result`](../../exercise-solutions/urls-match-result).
You can run them with the following command: `cargo run --example step_x`, where
x is the number of the step.

1. Fix the runtime error in the template code by correcting the file path. Then,
   handle the `Result` type that is returned from the
   `std::fs::read_to_string()` with a `match` block, instead of using
   `.unwrap()`.

2. Take the code from Step 1 and instead of using a `match`, propagate the Error
   with `?` out of `fn main()`. Note that your `main` function will now need to
   return something when it reaches the end.
  
3. Take the code from Step 2, and split the `String` into lines using the
   [`lines()`](https://doc.rust-lang.org/std/primitive.str.html#method.lines)
   method. Use this to count how many lines there are.

4. Change the code from Step 3 to filter out empty lines using
   [is\_empty](https://doc.rust-lang.org/std/primitive.str.html#method.is_empty)
   and print the non-empty ones.

5. Take your code from Step 4 and write a function like `fn parse_url(input: &str)
   -> Option<url::Url>` which checks if the given `input: &str` is a Url, or
   not. The function should return `Some(url)` where `url` is of type
   [Url](https://docs.rs/url/2.1.1/url/struct.Url.html), which is from the
   [`url` crate]. Use this function to convert each line and use the returned
   value to print either `Is a URL: <url>` or `Not a URL`.
   
   The [`url` crate] has already been added as a dependency so you can just use
   `url::Url::parse`

[`url` crate]: https://crates.io/crates/url

## Knowledge

### Option and Result

Both `Option` and `Result` are similar in a way. Both have two variants, and
depending on what those variants are, the program may continue in a different
way.

The Option type can have the variant `Some(T)` or `None`. `T` is a type
parameter that means some type should go here, we'll decide which one later. The
`Option` type is used when you have to handle optional values. For example if
you want to be able to leave a field of a struct empty, you use the `Option`
type for that field. If the field has a value, it is `Some(<value>)`, if it is
empty, it is `None`.

The variants of the `Result` type are `Ok(t)` and `Err(e)`. It is used to handle
errors. If an operation was successful, `Ok(t)` is returned. In `Ok(t)`, `t` can
be the empty tuple or some other value. In `Err(e)`, `e` contains an error
message that can usually be printed with `println!("Err: {:?}", e);`.

Both types can be used with the `match` keyword. The received value is matched
on patterns, each leads to the execution of one of a number of different
expressions depending on which arm matches first.

### How to use `match`

`match` is a way of control flow based on pattern matching. A pattern on the
left results in the expression on the right side.

 ```rust
let value = true;

match value {
    true => println!("This is true!"),
    false => println!("This is false!"),
}
```

Unlike with if/else, every case has to be handled explicitly, at least with a
last catch all arm that uses a place holder:

```rust
let value = 50_u32;

match value {
    1 => println!("This is one."),
    50 => println!("This is fifty!"),
    _ => println!("This is any other number from 0 to 4,294,967,295."),
}
```

There are different ways to use `match`:

The return values of the expression can be bound to a variable:

```rust
enum Season {
    Spring,
    Summer,
    Fall,
    Winter
}

fn which_season_is_now(season: Season) -> String {
    
    let return_value = match season {
        Season::Spring => String::from("It's spring!"),
        Season::Summer => String::from("It's summer!."),
        Season::Fall => String::from("It's Fall!"),
        Season::Winter => String::from("Brrr. It's Winter."),
    };
    
    return_value
}
```

In case of a `Result<T, E>`, match statements can be used to get to the inner value.

```rust, ignore
use std::fs::File;

fn main() {
    let file_result = File::open("hello.txt");

    let _file_itself = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Error opening the file: {:?}", error),
    };
}

```

All arms of the match tree have to either result in the same type, or they have
to *diverge* (that is, panic the program or return early from the function)!

## Template

Start your `VSCode` in the proper root folder to have
`Rust-Analyzer` working properly.

```shell
../../exercise-templates/urls-match-result/
```

The template builds, but has a runtime error, as the location of the file is
wrong. This is intentional.

Your code will use the example data found in

```shell
../../exercise-templates/urls-match-result/src/data
```

## Step-by-Step Solution

### Step 1: Handle the `Result` instead of unwrapping it

`std::fs::read_to_string` returns a `Result<T, E>` kind of type, a quick way to
get to inner type T is to use the `.unwrap()` method on the `Result<T, E>`. The
cost is that the program panics if the Error variant occurs and the Error can
not be propagated. It should only be used when the error does not need to be
propagated and would result in a panic anyways. It’s often used as a quick fix
before implementing proper error handling.

✅ Check the documentation for the exact type
    [std::fs::read_to_string](https://doc.rust-lang.org/std/fs/fn.read_to_string.html)
    returns.

✅ Handle the `Result` using `match` to get to the inner type. Link the two
possible patterns, `Ok(some_string)` and `Err(e)` to an an appropriate code
block, for example: `println!("File opened and read")` and `println!("Problem
opening the file: {:?}", e)`.

✅ Fix the path of the file so that the program no longer prints an error.

<details>
  <summary>Click me</summary>

```rust, ignore
{{#include ../../exercise-solutions/urls-match-result/examples/step_1.rs}}
```

</details>

TIP: IDEs often provide a "quick fix" to roll out all match arms quickly

### Step 2: Returning a Result from main

✅ Add `Result<(), Error>` as return type to `fn main()` and `Ok(())` as the last
line of `fn main()`.

✅ Delete the existing `match` block and add a `?` after the call to
`std::fs::read_to_string(...)`.

✅ Print something after the `std::fs::read_to_string` but before the `Ok(())` so
you can see that your program did run. Try changing the file path back to the
wrong value to see what happens if there is an error.

<details>
  <summary>Click me</summary>

```rust, ignore
{{#include ../../exercise-solutions/urls-match-result/examples/step_2.rs}}
```

</details>

### Step 3: Count the number of lines

✅ Take a look at the documentation of
[std::lines](https://doc.rust-lang.org/std/primitive.str.html#method.lines). It
returns a `struct Lines` which is an iterator. 

✅ Add a block like `for line in my_contents.lines() { }`

✅ Declare a mutable integer, initialised to zero. Increment that integer inside
the for loop.

✅ Print the number of lines the file contains.

<details>
  <summary>Click me</summary>

```rust, ignore
{{#include ../../exercise-solutions/urls-match-result/examples/step_3.rs}}
```

</details>

### Step 4: Filter out empty lines

✅ Filter out the empty lines, and only print the the others. The
[is\_empty](https://doc.rust-lang.org/std/string/struct.String.html#method.is_empty)
method can help you here.

<details>
  <summary>Click me</summary>

```rust, ignore
{{#include ../../exercise-solutions/urls-match-result/examples/step_4.rs}}
```

</details>

### Step 5: Check if a string is a URL, and return with `Option<T>`

✅ Write a function that takes `(input: &str)`, parses each line and returns
`Option<url::Url>` (using the [url::Url](https://docs.rs/url/2.1.1/url/)).
Search the docs for a method for this!

✅ If a line can be parsed successfully, return `Some(url)`, and return `None`
otherwise.

✅ In the `main` function, use your new function to only print value URLs.

✅ Test the `fn parse_url()`.

<details>
  <summary>Click me</summary>

```rust, ignore
{{#include ../../exercise-solutions/urls-match-result/examples/step_5.rs}}
```

</details>

## Help

### Typing variables

Variables can be typed by using `:` and a type.

```rust
let my_value: String = String::from("test");
```

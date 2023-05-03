# Files, Match and Result

In this exercise you will complete a number of mini exercises to learn about
Error Handling. The final result will be a url parser that reads lines
from a text file and can distinguish the content between URLs and
non-urls.

## In this exercise, you will learn how to

-   handle occurring `Result`-types with `match` for basic error
    handling.

-   when to use the `.unwrap()` method.

-   propagate an error with the `?` operator

-   return the `Option`-type.

-   do some elementary file processing (opening, reading to buffer,
    counting, reading line by line).

-   navigate the Rust `stdlib` documentation

-   add external dependencies to your project

## Task

Find the exercise template here [`../../exercise-templates/files-match-result`](../../exercise-templates/files-match-result)

Find the solution to the exercise here [`../../exercise-solutions/files-match-result`](../../exercise-solutions/files-match-result). You can run them with the following command:
`cargo run --example step_x`, where x is the number of the step. 


1. Fix the runtime error in the template code by correcting the file path. Handle the `Result` type that is returned from the `File::open()` with a match statement, so that the `.unwrap()` can be deleted. 

2. Read the content of the file to a buffer using [Read::read\_to\_string](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string). Propagate the Error with `?` to `fn main()`. Start with the code of Step 1. 
  
3. Take the code from Step 1 again, but now use the [`lines()`](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines)-method to instead read the file line-by-line from a [BufReader](https://doc.rust-lang.org/std/io/struct.BufReader.html). Use this to count how many lines there are.


4. Filter out empty lines using [is\_empty](https://doc.rust-lang.org/std/string/struct.String.html#method.is_empty) and print the non-empty ones. Start with the code of Step 3.

5. Write a function that parses each line and returns `Some(url)` if the line is a URL, and `None` if it is not. Use the Url crate's [UrlType](https://docs.rs/url/2.1.1/url/). Start with the code of Step 4.

## Knowledge

### Option and Result

Both `Option` and `Result` are similar in a way. Both have two
variants, and depending on what those variants are, the program may
continue in a different way.

The Option type can have the variant `Some(T)` or `None`. `T` is a type parameter that means some type should go here, we'll decide which one later.. It is used, when you have to handle optional values. For example
if you want to be able to leave a field of a struct empty, you assign the
`Option` type to it. If the field has a value, it is `Some(<value>)`, if
it is empty, it is `None`.

The variants of the `Result` type are `Ok(t)` and `Err(e)`. It is used to
handle errors. If an operation was successful, `Ok(t)` is returned. In
`Ok(t)`, `t` can be the empty tuple or a return value. In `Err(e)`, `e`
contains an error message that can be printed.

Both types can be used with the `match` keyword. The received value is
matched on patterns, each leads to the execution of a different
expression.

### How to use `match`

`match` is a way of control flow based on pattern matching. A pattern on
the left results in the expression on the right side.

 ```rust
let value = true;

match value {
    true => println!("This is true!"),
    false => println!("This is false!"),
}
```

Other then with if/else, every case has to be handled explicitly, at
least with a last catch all arm that uses a place holder:

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

In case of a `Result<T, E>`, match statements can be used to get to
the inner value.

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
All arms of the match tree have to either result in the same type, or they have to *diverge* (that is, panic the program or return early from the function)!

# Template

Start your `VSCode` in the proper root folder to have
`Rust-Analyzer` working properly.

```shell
$ ../../exercise-templates/files-match-result/
```

The template builds, but has a runtime error, as the location of the file
is wrong. This is intentional.

Your code will use the example data found in

```shell
$ ../../exercise-templates/files-match-result/src/data
```
## Step-by-Step Solution

### Step 1: Handle the `Result` instead of unwrapping it!

`File::open` yields a `Result<T, E>` kind of type, a quick way to get to
inner type T is to use the `.unwrap()` method on the `Result<T, E>`. The
cost is that the program panics if the Error variant occurs and the
Error can not be propagated. It should only be used when the error does
not need to be propagated and would result in a panic anyways. It’s
often used as a quick fix before implementing proper error handling.

✅ Check the documentation for the exact type
    [File::open](https://doc.rust-lang.org/std/fs/struct.File.html#method.open)
    returns.

✅ Handle the `Result` using `match` to get to the inner type. Link the two possible patterns, `Ok(file)` and `Err(e)` to an an appropriate expression, for example: `println!("File opened")` and `panic!("Problem opening the file: {:?}", e)`.

✅ Fix the location of the file so that the program no longer panics. 

<details>
  <summary>Click me</summary>

```rust, ignore
{{#include ../../exercise-solutions/files-match-result/examples/step_1.rs}}
```

</details>

TIP: IDEs often provide a "quick fix" to roll out all match arms quickly

### Step 2: Reading the File content to a String and Error propagation.

✅ Import `std::io::prelude::*`

Take a look at [Read::read\_to\_string](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string). The method takes in a mutable empty `String`, and writes the content of a file to this buffer. The method returns a `Result<usize, Error>`, where the usize is the number of bytes that have been written to the buffer. Handling this Result, will not yield the `String` of file content. For a simple program, handling it with an 
`.unwrap()` would be sufficient, but for bigger code bases this is not helpful, so we want to propagate the Error.

✅ Add `Result<(), Error>` as return type to `fn main()` and `Ok(())` as the last line of `fn main()`.
  

✅ Create an empty `String` that serves as buffer and bind it to a mutable variable. 

✅ Call the `.read_to_string()` method on the `File` object. The method takes in the `String` buffer and is followed by the `?` operator. If the method returns an `Error` it is propagated to the instance that called `fn main()`. If the method returns the `Ok` value, the program proceeds as planned.

✅ Use `println!` to print the content of the `String` buffer.

<details>
  <summary>Click me</summary>

```rust, ignore
{{#include ../../exercise-solutions/files-match-result/examples/step_2.rs}}
```
</details>

### Task 3: Read the lines into a `BufReader` and count them!

✅ Add the following imports:

```rust
use std::io::{ BufReader, BufRead,};
```

✅ Take a look at the documentation of [BufReader](https://doc.rust-lang.org/std/io/struct.BufReader.html). BufReader is a struct that adds buffering to any reader. It implements the
[`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html#) trait. In short this means, that methods that are defined for `BufRead` can be used for `BufReader`. For example the [`lines()`](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines) method.

✅ Construct a `BufReader` around the file.

✅ The `lines()`- method returns an Iterator over the file’s lines. Iterate over the lines with a for loop to count them.

✅ Print the number of lines the file contains.

✅ You don’t have to handle the `Result` that is returned from `.lines()`, why?

<details>
  <summary>Click me</summary>

```rust, ignore
{{#include ../../exercise-solutions/files-match-result/examples/step_3.rs}}
```

</details>

### Step 4: Filter out empty lines print the Rest …and Error Handling

✅ Filter out the empty lines, and only print the the others. The [is\_empty](https://doc.rust-lang.org/std/string/struct.String.html#method.is_empty) method can help you here.

<details>
  <summary>Click me</summary>

```rust, ignore
{{#include ../../exercise-solutions/files-match-result/examples/step_4.rs}}
```

</details>

### Step 5: Read URLs from file and return with `Option<T>`.

✅ Add `url = "2"` to your `[dependencies]` section in `Cargo.toml` and import `url::Url` in `main.rs`.

✅ Write a function that parses each line and returns `Option<Url>` using the [UrlType](https://docs.rs/url/2.1.1/url/). Search the docs for a method for this!

✅ If a line can be parsed successfully, return `Some(url)`, `None` otherwise.

✅ In the calling context, only print URLs that parse correctly.

✅ Test the `fn parse_url()`.

<details>
  <summary>Click me</summary>

```rust, ignore
{{#include ../../exercise-solutions/files-match-result/examples/step_5.rs}}
```

</details>

## Help

### Typing variables

Variables can be typed by using `:` and a type.

    let my_value: String = String::from("test");



# Replacing `println!`

In this exercise, we will write a basic "Hello, World!" application, but without using `println!`. This will introduce some of the concepts we will need for writing safety-critical Rust code that runs on certified OSes like QNX, where the Rust Standard Library is not available.

However, to keep things easy to deploy, you can use your normal Windows, macOS or Linux system to complete this exercise.

## Task 1 - Make a program

Use `cargo new` to make a package containing the default binary crate - a Hello, World example that uses `println!`

<details>
<summary>Solution</summary>

```console
$ cargo new testprogram
     Created binary (application) `testbin` package
$ cd testprogram
$ cargo run
   Compiling testbin v0.1.0 (/Users/jonathan/Documents/clients/training/oxidze-2024/testbin)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/testbin`
Hello, world!
```

</details>

## Task 2 - Lock the Standard Out

The `println!` expands to some code which:

1. Grabs a lock on standard out
2. Formats the arguments into the locked standard out

We can do these two steps manually, using [`std::io::stdout()`](https://doc.rust-lang.org/stable/std/io/fn.stdout.html), and the [`writeln!`](https://doc.rust-lang.org/stable/std/macro.writeln.html) (which is actually from in libcore).

Replace the call to `println!` with a call to `writeln!` that uses a locked standard out. Work out how best to handle the fact that `writeln!` returns an error. Think about why `println!` didn't return an error? How it did handle a possible failure?

If you get an error about the `write_fmt` method not being available, make sure you have brought the `std::io::Write` trait into scope. Recall that trait methods are not available on types unless the trait is in scope - otherwise how would the compiler know which traits to look for the method in? If we were on a no-std system, the same method is available in the `core::fmt::Write` trait - the `writeln!` macro is happy with either as long as the method exists.

<details>
<summary>Solution</summary>

```rust
use std::io::Write;

fn main() {
    let mut stdout = std::io::stdout();
    writeln!(stdout, "Hello, World!").expect("writing to stdout");
}
```

The `writeln` call can fail because the it can get an error from the object it is writing to. What if you are writing to a file on disk, and the disk is full? Or the USB Thumb Drive it is on is unplugged? The `println!` macro knows it only writes to Standard Out, and if that is broken, there isn't much you can do about it (you probably can't even print an error), so it just panics.

</details>

## Task 3 - Call `write_fmt`

The `writeln!` call expands to some code which:

1. Generates a value of type [`std::fmt::Arguments`](https://doc.rust-lang.org/stable/std/fmt/struct.Arguments.html), using a macro called `format_args!`.
2. Passes that to the `write_fmt` method on whatever we're writing into.

You can do these two steps manually - but that's as far as we can go! The `format_args!` macro is special, and we are unable to replicate its functions by writing regular Rust code.

Replace the call to `writeln!` with a call to `format_args!`, passing the result to the `write_fmt` method on the locked standard output. Note that Rust won't let you store the result of `format_args!` in a variable - you need to call it inside the call to `write_fmt`. Try it for yourself!

<details>
<summary>Solution</summary>

```rust
use std::io::Write;

fn main() {
    let mut stdout = std::io::stdout();
    stdout.write_fmt(format_args!("Hello, World!"));
}
```

</details>

## Task 4 - Ditch the standard output object

Rather than throw bytes into this mysterious *Standard Out* object, let's try and talk to our Operating System directly. We're going to do this using the [`libc`](https://docs.rs/libc) crate, which provides raw access to the APIs typically found in most C Standard Libraries.

* Step 1 - Run `cargo add libc` to add it as a dependency
* Step 2 - Store your message in a local variable, as a string slice

  ```rust
  let message = "Hello, World!";
  ```

* Step 3 - Unsafely call the `libc::write` method, passing:
  * `1` as the file descriptor (the standard output has this value, by default)
  * A pointer to the start of your string slice
  * The length of the string in bytes

You can make a pointer from a slice using the `as_ptr()` method, but this will give you `*const u8` and `libc::write` might want `*const c_void`. You can use `message.as_ptr() as _` to get Rust to cast the pointer into an automatically determined type (the `_` means 'work this out for me').

You might also find the length of the string needs casting from the default `usize` to whatever libc wants on your platform.

<details>
<summary>Solution</summary>

```rust ignore
fn main() {
    let message = "Hello, world";
    unsafe {
        libc::write(1, message.as_ptr() as _, message.len() as _);
    }
}
```

</details>

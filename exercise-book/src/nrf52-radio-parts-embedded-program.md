# Parts of an Embedded Program

We will look at the elements that distinguish an embedded Rust program from a desktop program.

✅ Open the `nrf52-code/radio-app` folder in VS Code.

```sh
# or use "File > Open Folder" in VS Code
code nrf52-code/radio-app
```

✅ Then open the `nrf52-code/radio-app/src/bin/hello.rs` file.

## Attributes

In the file, you will find the following attributes:

### `#![no_std]`

 The `#![no_std]` language attribute indicates that the program will not make use of the standard library, the `std` crate. Instead it will use the `core` library, a subset of the standard library that does not depend on an underlying operating system (OS).

### `#![no_main]`

The `#![no_main]` language attribute indicates that the program will use a custom entry point instead of the default `fn main() { .. }` one.

### `#[entry]`

The `#[entry]` macro attribute marks the custom entry point of the program. The entry point must be a divergent function whose return type is the never type `!`. The function is not allowed to return; therefore the program is not allowed to terminate. The macro comes from the [cortex-m-rt crate](https://docs.rs/cortex-m-rt/0.7.3/cortex_m_rt/attr.entry.html) and is not part of the Rust language.

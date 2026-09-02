# Building an Embedded Program

The default in a Cargo project is to compile for the host (native compilation).
The [`stm32-code/standalone-app`](../../../stm32-code/standalone-app) project
has been configured for cross compilation to the ARM Cortex-M33 processor. This
configuration can be seen in the Cargo configuration file (`.cargo/config`):

```text
# .cargo/config
[build]
target = "thumbv8m.main-none-eabi" # = ARM Cortex-M33
```

The target `thumbv8m.main-none-eabi` can be broken down as:

* `thumbv8m.main` - we generate instructions for the Armv8-M Mainline
  architecture running in Thumb-2 mode (actually the only supported mode on this
  architecture)
* `none` - there is no Operating System
* `eabi` - use the ARM *Embedded Application Binary Interface*, with *Soft
  Float* support
  * `f32` and `f64` are passed to functions in normal CPU registers (like `R0`),
    instead of in FPU registers (like `S0`)

✅ Inside the folder [`stm32-code`](../../../stm32-code/), use the following
command to cross compile the program:

```console
cargo build --bin standalone-hello
```

Building the application also requires a Rust Standard Library that has been
pre-compiled for the `thumbv8m.main-none-eabi` target. Normally, you would have
to add this component to `cargo` by using
`rustup target add thumbv8m.main-none-eabi`, but we provide a
`rust-toolchain.toml` that is used to determine which toolchains and targets
should be installed automatically when in this folder.

The output of the compilation process will be an ELF (Executable and Linkable
Format) file. The file will be placed in the `target/thumbv8m.main-none-eabi`
directory.

✅ Run `file target/thumbv8m.main-none-eabi/debug/standalone-hello` and compare
if your output is as expected.

Expected output:

```console
$ file target/thumbv8m.main-none-eabi/debug/standalone-hello
target/thumbv8m.main-none-eabi/debug/standalone-hello: ELF 32-bit LSB executable, ARM, EABI5 version 1 (GNU/Linux), statically linked, with debug_info, not stripped
```

# Building an Embedded Program

The default in a Cargo project is to compile for the host (native compilation). The [`nrf52-code/radio-app`](../../nrf52-code/radio-app) project has been configured for cross compilation to the ARM Cortex-M4 architecture. This configuration can be seen in the Cargo configuration file (`.cargo/config`):

```text
# .cargo/config
[build]
target = "thumbv7em-none-eabihf" # = ARM Cortex-M4
```

The target `thumbv7em-none-eabihf` can be broken down as:

* `thumbv7em` - we generate instructions for the Armv7E-M architecture running in Thumb-2 mode (actually the only supported mode on this architecture)
* `none` - there is no Operating System
* `eabihf` - use the ARM *Embedded Application Binary Interface*, with *Hard Float* support
  * `f32` and `f64` can be passed to functions in FPU registers (like `S0`), instead of in integer registers (like `R0`)

✅ Inside the folder [`nrf52-code/radio-app`](../../nrf52-code/radio-app), use the following command to cross compile the program to the ARM Cortex-M4 architecture.

```console
cargo build --bin hello
```

The output of the compilation process will be an ELF (Executable and Linkable Format) file. The file will be placed in the `target/thumbv7em-none-eabihf` directory.

✅ Run `$ file target/thumbv7em-none-eabihf/debug/hello` and compare if your output is as expected.

Expected output:

```console
$ file target/thumbv7em-none-eabihf/debug/hello
hello: ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), statically linked, with debug_info, not stripped
```

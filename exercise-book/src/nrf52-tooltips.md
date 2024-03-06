# Tooltips

Besides the ones covered in this workshop, there are many more tools that make embedded development easier.
Here, we'd like to introduce you to some of these tools and encourage you to play around with them and adopt them if you find them helpful!

## `cargo-bloat`

`cargo-bloat` is a useful tool to analyze the binary size of a program. You can install it through cargo:

```console
$ cargo install cargo-bloat
(..)
Installed package `cargo-bloat v0.10.0` (..)
```

Let's inspect our radio workshop's `hello` program with it:

```console
$ cd nrf52-code/radio-app
$ cargo bloat --bin hello
File  .text   Size      Crate Name
0.7%  13.5% 1.3KiB        std <char as core::fmt::Debug>::fmt
0.5%   9.6%   928B      hello hello::__cortex_m_rt_main
0.4%   8.4%   804B        std core::str::slice_error_fail
0.4%   8.0%   768B        std core::fmt::Formatter::pad
0.3%   6.4%   614B        std core::fmt::num::<impl core::fmt::Debug for usize>::fmt
(..)
5.1% 100.0% 9.4KiB            .text section size, the file size is 184.5KiB
```

This breaks down the size of the `.text` section by function. This breakdown can be used to identify the largest functions in the program; those could then be modified to make them smaller.

## Using `probe-rs` VS Code plugin

The [probe-rs](https://probe.rs) team have produced a VS Code plugin. It uses the `probe-rs` library to talk directly to your supported Debug Probe (J-Link, ST-Link, CMSIS-DAP, or whatever) and supports both single-stepping and `defmt` logging.

Install the `probe-rs.probe-rs-debugger` extension in VS Studio, and when you open the [`nrf52-code/radio-app`](../../nrf52-code/radio-app) folder in VS Code, the `.vscode/launch.json` file we supply should give you a *Run with probe-rs* entry in the *Run and Debug* panel. Press the green triangle and it will build the code, flash device, set up defmt and then start the chip running. You can set breakpoints in the usual way (by clicking to the left of your source code to place a red dot).

## Using `gdb` and `probe-rs`

The CLI `probe-rs` command has an option for opening a GDB server. We have found the command-line version of GDB to be a little buggy though, so the VS Code plugin above is preferred.

```console
$ probe-rs gdb --chip nRF52840_xxAA
# In another window
$ arm-none-eabi-gdb ./target/thumbv7em-none-eabihf/debug/blinky
gdb> target extended-remote :1337
gdb> monitor reset halt
gdb> break main
gdb> continue
Breakpoint 1, blinky::__cortex_m_rt_main_trampoline () at src/bin/blinky.rs:10
```

## Using `gdb` and `openocd`

You can also debug a Rust program using `gdb` and `openocd`. However, this isn't recommended because it requires significant extra set-up, especially to get the RTT data piped out of a socket and into `defmt-print` (this function is built into a `probe-rs`).

If you are familiar with OpenOCD and GDB, and want to try this anyway, then do pretty much what you would do with a C program.

The only change is that if you want defmt output, you need these OpenOCD commands to enable RTT:

```text
rtt setup 0x20000000 0x40000 "SEGGER RTT"
rtt start
rtt server start 9090 0
```

You can then use `nc` to connect to `localhost:9090`, and pipe the output into `defmt-print`:

```sh
nc localhost:9090 | defmt-print ./target/thumbv7em-none-eabihf/debug/blinky
```

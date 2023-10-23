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

## Using `gdb`

To debug embedded Rust applications with GDB we currently recommend using tooling like OpenOCD,
JLinkGDBServer or pyOCD

Although `cargo-embed` v0.10 (and v0.10 of `probe-rs`, the library that powers `cargo-embed`) support spawning a GDB server it has some limitations

- stepping through the code (e.g. GDB's `step` and `next` commands) is imprecise or doesn't work in
  some cases
- it's not possible to have a GDB server and RTT channels running at the same time so you can use GDB OR RTT but not both together (this limitation is likely to be removed in v0.11)

The rest of this section covers how to debug an embedded application within VS code using OpenOCD as
the GDB server.

### Dependencies

0. Make sure you've connected your Development Kit: USB port J2 on the board

1. You'll need to install OpenOCD. Installation instructions vary depending on your OS.

2. Install the [cortex-debug](https://marketplace.visualstudio.com/items?itemName=marus25.cortex-debug) extension in VS code.

### Preparation

For the best debugging experience, the `dev` (development) compilation profile should be set to its
default settings.
For this tutorial, we'll be using the `nrf52-code/` applications, so let's modify `nrf52-code/radio-app/Cargo.toml` to revert the `dev` profile to its default.

```diff
 panic-log = { path = "../../common/panic-log" }

 # optimize code in both profiles
-[profile.dev]
-codegen-units = 1
-debug = 1
-debug-assertions = true # !
-incremental = false
-lto = "fat"
-opt-level = 'z' # !
-overflow-checks = false

 [profile.release]
```

### How to

1. In VS code, from the top menu pick "File" > "Open folder". Then open the `nrf52-code/radio-app` folder.

2. Within this folder, open the `src/bin/hello.rs` file.

3. From the top menu, pick "Run" > "Start Debugging".

[![GDB session within VS code using the cortex-debug extension](code-gdb.png)](./code-gdb.png)

You are now in a GDB session. Switch to the "Run" view (4th icon from the top on the left sidebar),
if VS code didn't automatically switch to it, and you'll see debug information like the call stack,
local variables, breakpoints and CPU registers on the left side. On the bottom panel, you can switch
to the "Debug console" to issue commands to the GDB server. Near the top of the GUI you'll find a
row of buttons to navigate through the program (step, continue, etc.). Breakpoints can be added by
clicking to the left of line numbers in the file view.

## Debugging a different program

To debug a different program within the `nrf52-code/radio-app` folder you'll need to modify the
`nrf52-code/radio-app/.vscode/launch.json` file as follows:

```diff
 {
     "version": "0.2.0",
     "configurations": [
       {
         "cwd": "${workspaceRoot}",
-        // TODO to debug a different program the app name ("hello") needs to be changed
-        "executable": "./target/thumbv7em-none-eabihf/debug/hello",
+        "executable": "./target/thumbv7em-none-eabihf/debug/blinky",
         "name": "Debug Microcontroller (launch)",
```

Change the name of the program from `hello` to whichever program you wish to debug.

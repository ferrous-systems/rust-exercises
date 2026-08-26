# Running the Program

## Setting the log level

Enter the appropriate command into the terminal you're using. This will set the log level for this session.

### MacOS & Linux

```console
export DEFMT_LOG=warn
```

### PowerShell

```console
$Env:DEFMT_LOG = "warn"
```

### Windows Command Prompt

```console
set DEFMT_LOG=warn
```

### Inside VS Code

To get VS Code to pick up the environment variable, you can either:

* set it as above and then open VS Code from inside the terminal (ensuring it wasn't already open and hence just getting you a new window on the existing process), or
* add it to your rust-analyzer configuration, by placing this in your `settings.json` file:

  ```json
  "rust-analyzer.runnables.extraEnv": {
      "DEFMT_LOG": "warn"
  }
  ```

  This will ensure the variable is set whenever rust-analyzer executes `cargo run` for you.

## Running from VS Code

✅ Open the [`stm32-code/standalone-app/src/bin/standalone-hello.rs`](../../../stm32-code/standalone-app/src/bin/standalone-hello.rs) file, go to the "Run and Debug" button on the left, and then click the "Run" triangle next to *Debug Microcontroller*.

> Note: you will get the "Run" button if the Rust analyzer's workspace is set to the [`stm32-code`](../../../stm32-code) folder. This will be the case if the current folder in VS code (left side panel) is set to [`stm32-code`](../../../stm32-code).

## Running from the console

If you are not using VS code, you can run the program out of your console. Enter the command `cargo run --bin standalone-hello` from within the [`stm32-code`](../../../stm32-code) folder. Rust Analyzer's "Run" button is a short-cut for that command.

## Expected output

```console
$ cargo run --bin standalone-hello
   Compiling standalone-app v0.1.0 (/Users/jonathan/Documents/ferrous-systems/rust-exercises/stm32-code/standalone-app)
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.17s
     Running `probe-rs run --chip STM32U5A5ZJ target/thumbv8m.main-none-eabi/debug/standalone-hello`
      Erasing ✔ 100% [####################]  16.00 KiB @ 164.27 KiB/s (took 0s)
  Programming ✔ 100% [####################]  12.00 KiB @  29.97 KiB/s (took 0s)
     Finished in 0.60s
Hello, world!
```

## What just happened?

`cargo run` will compile the application and then invoke the `probe-rs` tool with its final argument set to the path of the output ELF file.

The `probe-rs` tool will

* flash (load) the program on the microcontroller
* reset the microcontroller to make it execute the new program
* collect logs from the microcontroller and print them to the console
* print a backtrace of the program if the halt was due to an error.

Should you need to configure the `probe-rs` invocation to e.g. flash a different microcontroller you can do that in the `.cargo/config.toml` file.

```toml
[target.thumbv8m.main-none-eabi]
runner = "probe-rs run --chip STM32U5A5ZJ"
# ..
```

**🔎 How does flashing work?**

The flashing process consists of the PC communicating with a second microcontroller on the NUCLEO-U5A5 board over USB (the `STLK` port). This second microcontroller, which is a *ST-Link Arm Debug Probe*, is connected to the STM32U5A5 through a electrical interface known as *SWD* (Serial Wire Debug). The *SWD* protocol specifies procedures for reading memory, writing to memory, halting the target processor, reading the target processor registers, etc.

**🔎 How does logging work?**

Logging is implemented using the Real Time Transfer (RTT) protocol. Under this protocol the target device writes log messages to a ring buffer stored in RAM; the PC communicates with the J-Link to read out log messages from this ring buffer. This logging approach is non-blocking in the sense that the target device does not have to wait for physical IO (USB comm, serial interface, etc.) to complete while logging messages since they are written to memory. It is possible, however, for the target device to run out of space in its logging ring buffer; this causes old log messages to be overwritten or the microcontroller to pause whilst waiting for the PC to catch up with reading messages (depending on configuration).

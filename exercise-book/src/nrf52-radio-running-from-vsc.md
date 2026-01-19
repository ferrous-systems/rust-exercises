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

✅ Open the [`nrf52-code/radio-app/src/bin/hello.rs`](../../nrf52-code/radio-app/src/bin/hello.rs) file, go to the "Run and Debug" button on the left, and then click the "Run" triangle next to *Debug Microcontroller*.

> Note: you will get the "Run" button if the Rust analyzer's workspace is set to the [`nrf52-code/radio-app`](../../nrf52-code/radio-app) folder. This will be the case if the current folder in VS code (left side panel) is set to [`nrf52-code/radio-app`](../../nrf52-code/radio-app).

## Running from the console

If you are not using VS code, you can run the program out of your console. Enter the command `cargo run --bin hello` from within the [`nrf52-code/radio-app`](../../nrf52-code/radio-app) folder. Rust Analyzer's "Run" button is a short-cut for that command.

## Expected output

> __NOTE:__ Recent version of the nRF52840-DK have flash-read-out protection to stop people dumping the contents of flash on an nRF52 they received pre-programmed, so if you have problems immediately after first plugging your board in, see [this page](./nrf52-tools.md#setup-check).
>
> More recent versions of the DK board also have configuration parameters which need to be updated inside
> a special segment of the flash, which in turn requires a soft-reset. This needs to be done for reset pin
> configuration and for allowing debugging. The firmware will generally perform this task, but the
> interaction with previously flashed software might lead to unexpected errors on the first flash operation
> of a fresh board.
>
> If you run into an error along the lines of "Debug power request failed" or "Firmware exited unexpectedly: Exception",
> retry the operation and the error should disappear.

```console
$ cargo run --bin hello
   Compiling radio_app v0.0.0 (/Users/jonathan/Documents/ferrous-systems/rust-exercises/nrf52-code/radio-app)
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.36s
     Running `probe-rs run --chip=nRF52840_xxAA --allow-erase-all --log-format=oneline target/thumbv7em-none-eabihf/debug/hello`
      Erasing ✔ 100% [####################]  12.00 KiB @  18.70 KiB/s (took 1s)
  Programming ✔ 100% [####################]  12.00 KiB @  14.34 KiB/s (took 1s)
  Finished in 1.48s
Hello, world!
`dk::exit()` called; exiting ...
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
[target.thumbv7em-none-eabihf]
runner = [
  "probe-rs",
  "run",
  "--chip",
  "nRF52840_xxAA"
]
# ..
```

**🔎 How does flashing work?**

The flashing process consists of the PC communicating with a second microcontroller on the nRF52840 DK over USB (J2 port). This second microcontroller, which is a *J-Link Arm Debug Probe*, is connected to the nRF52840 through a electrical interface known as *SWD* (Serial Wire Debug). The *SWD* protocol specifies procedures for reading memory, writing to memory, halting the target processor, reading the target processor registers, etc.

**🔎 How does logging work?**

Logging is implemented using the Real Time Transfer (RTT) protocol. Under this protocol the target device writes log messages to a ring buffer stored in RAM; the PC communicates with the J-Link to read out log messages from this ring buffer. This logging approach is non-blocking in the sense that the target device does not have to wait for physical IO (USB comm, serial interface, etc.) to complete while logging messages since they are written to memory. It is possible, however, for the target device to run out of space in its logging ring buffer; this causes old log messages to be overwritten or the microcontroller to pause whilst waiting for the PC to catch up with reading messages (depending on configuration).

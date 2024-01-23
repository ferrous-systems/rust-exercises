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

## Run from VS Code

✅ Open the `nrf52-code/radio-app/src/bin/hello.rs` file, go to the "Run and Debug" button on the left, and then click the "Run" triangle next to *Debug Microcontroller*.

> Note: you will get the "Run" button if the Rust analyzer's workspace is set to the `nrf52-code/radio-app` folder. This will be the case if the current folder in VS code (left side panel) is set to `nrf52-code/radio-app`.

If you are not using VS code, you can run the program out of your console. Enter the command `cargo run --bin hello` from within the `nrf52-code/radio-app` folder. Rust Analyzer's "Run" button is a short-cut for that command.

> NOTE: If you run into an error along the lines of "Debug power request failed" retry the operation and the error should disappear.

Expected output:

```console
$ cargo run --bin hello
   Compiling radio_app v0.0.0 (/Users/jonathan/Documents/ferrous-systems/rust-exercises/nrf52-code/radio-app)
    Finished dev [optimized + debuginfo] target(s) in 0.28s
     Running `probe-rs run --chip nRF52840_xxAA target/thumbv7em-none-eabihf/debug/hello`
     Erasing sectors ✔ [00:00:00] [######################################################] 8.00 KiB/8.00 KiB @ 26.71 KiB/s (eta 0s )
 Programming pages   ✔ [00:00:00] [######################################################] 8.00 KiB/8.00 KiB @ 29.70 KiB/s (eta 0s )    Finished in 0.59s
Hello, world!
`dk::exit()` called; exiting ...
```

`cargo run` will compile the application and then invoke the `probe-rs` tool with its argument set to the path of the output ELF file.

The `probe-rs` tool will

- flash (load) the program on the microcontroller
- reset the microcontroller to make it execute the new program
- collect logs from the microcontroller and print them to the console
- print a backtrace of the program if the halt was due to an error.

Should you need to configure the `probe-rs` invocation to e.g. flash a different microcontroller you can do that in the `.cargo/config.toml` file.

```toml
[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip nRF52840_xxAA" # <- add/remove/modify flags here
# ..
```

**🔎 How does flashing work?**

The flashing process consists of the PC communicating with a second microcontroller on the nRF52840 DK over USB (J2 port). This second microcontroller, which is a *J-Link Arm Debug Probe*, is connected to the nRF52840 through a electrical interface known as *SWD* (Serial Wire Debug). The *SWD* protocol specifies procedures for reading memory, writing to memory, halting the target processor, reading the target processor registers, etc.

**🔎 How does logging work?**

Logging is implemented using the Real Time Transfer (RTT) protocol. Under this protocol the target device writes log messages to a ring buffer stored in RAM; the PC communicates with the J-Link to read out log messages from this ring buffer. This logging approach is non-blocking in the sense that the target device does not have to wait for physical IO (USB comm, serial interface, etc.) to complete while logging messages since they are written to memory. It is possible, however, for the target device to run out of space in its logging ring buffer; this causes old log messages to be overwritten or the microcontroller to pause whilst waiting for the PC to catch up with reading messages (depending on configuration).

# Hello, world!

In this section, we'll set up the integration in VS Code and run the first program.

✅ Open the `nrf52-code/usb-app` folder in VS Code and open the `src/bin/hello.rs` file.

> Note: To ensure full rust-analyzer support, do not open the whole `rust-exercises` folder.

Give rust-analyzer some time to analyze the file and its dependency graph. When it's done, a "Run" button will appear over the `main` function. If it doesn't appear on its own, type something in the file, delete and save. This should trigger a re-load.

✅ Click the "Run" button to run the application on the microcontroller.

If you are not using VS code run the `cargo run --bin hello` command from the `nrf52-code/usb-app` folder.

> __NOTE:__ Recent version of the nRF52840-DK have flash-read-out protection to stop people dumping the contents of flash on an nRF52 they received pre-programmed, so if you have problems immediately after first plugging your board in, see [this page](./nrf52-tools.md#setup-check).
>
> If you run into an error along the lines of "Debug power request failed" retry the operation and the error should disappear.

The `usb-app` package has been configured to cross-compile applications to the ARM Cortex-M architecture and then run them using the `probe-rs` custom Cargo runner. The `probe-rs` tool will load and run the embedded application on the microcontroller and collect logs from the microcontroller.

The `probe-rs` process will terminate when the microcontroller enters the "halted" state. From the embedded application, one can enter the "halted" state using by performing a CPU breakpoint with a special argument that indicates 'success'. For convenience, an `exit` function is provided in the `dk` Board Support Package (BSP). This function is divergent like `std::process::exit` (`fn() -> !`) and can be used to halt the device and terminate the `probe-rs` process.

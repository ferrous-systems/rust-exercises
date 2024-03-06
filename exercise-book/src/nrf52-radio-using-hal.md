# Using a Hardware Abstraction Layer

✅ Open the [`nrf52-code/radio-app/src/bin/led.rs`](../../nrf52-code/radio-app/src/bin/led.rs) file.

You'll see that it initializes your board using the `dk` crate:

```rust ignore
let board = dk::init().unwrap();
```

This grants you access to the board's peripherals, like its LEDs.

The `dk` crate / library is a Board Support Package (BSP) tailored to this workshop to make accessing the peripherals used in this workshop extra seamless. You can find its source code at [`nrf52-code/boards/dk/src/`](../../nrf52-code/boards/dk/src/).

`dk` is based on the [`nrf52840-hal`] crate, which is a Hardware Abstraction Layer (HAL) over the nRF52840 System on Chip. The purpose of a HAL is to abstract away the device-specific details of the hardware, for example registers, and instead expose a higher level API more suitable for application development.

The `dk::init` function we have been calling in all programs initializes a few of the nRF52840 peripherals and returns a `Board` structure that provides access to those peripherals. We'll first look at the `Leds` API.

✅ Run the `led` program. Two of the green LEDs on the board should turn on; the other two should stay off.

> NOTE this program will not terminate itself. Within VS code you need to click "Kill terminal" (garbage bin icon) in the bottom panel to terminate it.

✅ Open the documentation for the `dk` crate by running the following command from the [`nrf52-code/radio-app`](../../nrf52-code/radio-app) folder:

```console
cargo doc -p dk --open
```

✅ Check the API docs of the `Led` abstraction. Change the `led` program, so that the bottom two LEDs are turned on, and the top two are turned off.

🔎 If you want to see logs from Led API of the `dk` Board Support Package, flash the dk with the following environment variable:

```console
DEFMT_LOG=trace cargo run --bin led
```

The logs will appear on your console, as the output of `cargo run`. Among the logs you'll find the line "I/O pins have been configured for digital output". At this point the electrical pins of the nRF52840 microcontroller have been configured to drive the 4 LEDs on the board.

After the `dk::init` logs you'll find logs about the `Led` API. As the logs indicate, an LED becomes active when the output of the pin is a *logical zero*, which is also referred as the "low" state. This "active low" configuration does not apply to all boards: it depends on how the pins have been wired to the LEDs. You should refer to the [board documentation] to find out which pins are connected to LEDs and whether "active low" or "active high" applies to it.

🔎 When writing your own embedded project, you can implement your own BSP similar to `dk`, or use the matching HAL crate for your chip directly. Check out [awesome-embedded-rust] if there's a BSP for the board you want to use, or a HAL crate for the chip you'd like to use.

[`nrf52840-hal`]: https://docs.rs/nrf52840-hal/0.12.1/nrf52840_hal/
[board documentation]: https://infocenter.nordicsemi.com/index.jsp?topic=%2Fug_nrf52840_dk%2FUG%2Fnrf52840_DK%2Fintro.html
[awesome-embedded-rust]: https://github.com/rust-embedded/awesome-embedded-rust#hal-implementation-crates

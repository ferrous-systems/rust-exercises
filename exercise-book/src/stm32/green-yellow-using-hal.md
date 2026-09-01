# Using a Hardware Abstraction Layer

✅ Open the [`stm32-code/standalone-app/src/bin/standalone-led.rs`](../../../stm32-code/standalone-app/src/bin/standalone-led.rs) file.

You'll see that it initializes your board using the `nucleo_u5a5zj_bsp` crate:

```rust ignore
let board = nucleo_u5a5zj_bsp::NonSecureBoard::new();
```

This grants you access to the board's peripherals, like its LEDs.

The `nucleo-u5a5zj-bsp` library is a Board Support Package (BSP) tailored to this training to make accessing the peripherals used in this exercise as seamless as possible. You can find its source code at [`stm32-code/nucleo-u5a5zj-bsp/`](../../../stm32-code/nucleo-u5a5zj-bsp/).

`nucleo-u5a5zj-bsp` is based on a custom *Hardware Abstraction Layer* we wrote for the STM32U5A5, called `stm32u5-tiny-hal`. The purpose of a HAL is to abstract away the device-specific details of the hardware, for example registers, and instead expose a higher level API more suitable for application development. Again, this one was designed to be small and easily understood and provide just enough support for these exercises. You can find its source code at [`stm32-code/stm32u5-tiny-hal/`](../../../stm32-code/stm32u5-tiny-hal/). 

The `nucleo_u5a5zj_bsp::NonSecureBoard::new` function we have been calling in all programs initializes a few of the STM32U5A5's peripherals and returns a `NonSecureBoard` structure that provides access to those peripherals. We'll first look at the `Leds` API.

✅ Run the `standalone-led` program. Two of the LEDs on the board should turn on; the other one should stay off.

> NOTE this program will not terminate itself. Within VS code you need to click "Kill terminal" (garbage bin icon) in the bottom panel to terminate it.

✅ Open the documentation for the `nucleo-u5a5zj-bsp` crate by running the following command from the [`stm32-code`](../../stm32-code) folder:

```console
cargo doc -p nucleo-u5a5zj-bsp --open
```

✅ Check the API docs of the `Led` abstraction, and look at its source code. Change the `standalone-led` program, so that the Red and Blue LEDs are turned on, and the Green LED is turned off.

Note that on this board, turning an LED "on" means setting its corresponding GPIO pin "high". On other boards you might set the GPIO pin "low" to turn the LED "on". You should refer to the [board documentation] to find out how your board is configured.

🔎 When writing your own embedded project, you can implement your own BSP similar to `nucleo-u5a5zj-bsp`, or use the matching HAL crate for your chip directly. Check out [awesome-embedded-rust] if there's a BSP for the board you want to use, or a HAL crate for the chip you'd like to use. Or you can even write a HAL from scratch, like we did!

[board documentation]: https://www.st.com/resource/en/user_manual/um2861-stm32u5-nucleo144-board-mb1549-stmicroelectronics.pdf
[awesome-embedded-rust]: https://github.com/rust-embedded/awesome-embedded-rust#hal-implementation-crates

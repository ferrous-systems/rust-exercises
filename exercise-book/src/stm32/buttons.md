# STM32 Buttons Exercise

In this exercise you'll get familiar with:

- controlling GPIO pins from a Rust HAL
- modifying a Board Support Package (BSP) to configure additional hardware

## Outline

As discussed in the previous exercise, the [`nucleo-u5a5zj-bsp` BSP] configures the three GPIO pins that control the three LEDs present on the NUCLEO-U5A5ZJ-Q board.

In this exercise we're going to extend the BSP to support support the "User" button. The `NonSecureBoard` structure should have a new field `button` and the data type for that field should have an API `fn is_pressed(&self) -> bool` which reports whether the button is pressed.

Once we've added that API, we will modify the [`standalone-button.rs`] file to do something interesting when the button is pressed. Exactly what is up to you!

[`nucleo-u5a5zj-bsp` BSP]: ../../../stm32-code/nucleo-u5a5zj-bsp/
[`standalone-button.rs`]: ../../../stm32-code/standalone-app/src/bin/standalone-button.rs

## Tasks

1. Study the `Led` type in the [`nucleo-u5a5zj-bsp` BSP]
1. Add a new `Button` type to the BSP
1. Give the `Button` type a `fn is_pressed(&self) -> bool` method
1. Add a field of type `Button` to the `NonSecureBoard` structure
1. Review the [STM32 NUCLEO documentation] to check which GPIO pin the "User" button (B1) is connected to and whether the pin goes high or low when pressed
1. Initialise the value of type `Button` and add it to the  `NonSecureBoard` initialisation

[STM32 NUCLEO documentation]: https://www.st.com/resource/en/user_manual/um2861-stm32u5-nucleo144-board-mb1549-stmicroelectronics.pdf

## Step by Step

### The Button Type

You should end up with something like:

<details>
<summary>Solution</summary>

```rust,ignore
/// Represents a Button on the board
pub struct Button {
    inner: Input,
}

impl Button {
    /// Is the button pressed?
    pub fn is_pressed(&self) -> bool {
        self.inner.is_high()
    }
}
```

Import `Input` as required.

The GPIO goes "high" when pressed.

</details>

### Reading the User Manual

<details>
<summary>Solution</summary>

Section 7.6 explains that User button (B1):

- is on pin PC13
- requires a pull-down input
- goes high when pressed

Observe the warning about never setting this pin to an output!

</details>


### Modifying the `NonSecureBoard`

Once you added a field, like `user_button: Button`, you'll need to create a value of type `Button`.

<details>
<summary>Solution</summary>

```rust,ignore
let user_button = Button {
    inner: gpio.change_to_input(pins.port_c.pin13, Pull::Down),
};
```

Import `Pull` as required.

</details>


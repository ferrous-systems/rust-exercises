# Adding Buttons

To practice using a HAL to provide functionality through a Board Support Package, you will now modify the `dk` crate to add support for Buttons.

## Change the demo app

✅ Change the `hal-app/src/bin/buttons.rs` file as described within, so it looks for button presses.

It should now fail to compile, because the `dk` crate doesn't have support for buttons. You will now fix that!

## Define a Button

✅ Open up the `dk` crate in VS Code (`nrf52-code/boards/dk`) and open `src/lib.rs`.

✅ Add a `struct Button` which represents a single button.

It should be similar to `struct Led`, except the inner type must be `Pin<Input<PullUp>>`. You will need to import those types - look where `Output` and `PushPull` types were imported from for clues! Think about where it makes sense to add this new type. At the top? At the bottom? Maybe just after to the LED related types?

🔎 The pins must be set as pull-ups is because each button connects a GPIO pin to ground, but the pins float when the button is not pressed. Enabling the pull-ups inside the SoC ensure that the GPIO pin is weakly connected to 3.3V through a resistor, giving it a 'default' value of 'high'. Pressing the button then makes the pin go 'low.

## Define all the Buttons

✅ Add a `struct Buttons` which contains four buttons.

Use `struct Leds` for guidance. Add a `buttons` field to `struct Board` which is of type `Buttons`. Again, think about where it makes sense to insert this new field.

## Set up the buttons

Now the `Board` struct initialiser is complaining you didn't initialise the new `buttons` field.

✅ Take pins from the HAL, configure them as inputs with pull-ups, and install them into the Buttons structure.

The mapping is:

* Button 1: P0.11
* Button 2: P0.12
* Button 3: P0.24
* Button 4: P0.25

You can verify this in the [User Guide](https://docs.nordicsemi.com/bundle/ug_nrf52840_dk/page/UG/dk/intro.html).

## Run your program

✅ Run the `buttons` demo:

```console
cd nrf52-code/hal-app
cargo run --bin buttons
```

Now when you press the button, the LED should illuminate. If it does the opposite, check your working!

## Write a more interesting demo program for the BSP

✅ You've got four buttons and four LEDs. Make up a demo!

If you're stuck for ideas, you could have the LEDs do some kind of animation. The buttons might then stop or start the animation, or make it go faster or slower. Try setting up a loop with a 20ms delay inside it, to give yourself a basic 50 Hz "game tick". You can look at the `blinky` demo for help with the timer.

## Troubleshooting

🔎 If you get totally stuck, ask for help! If all else fails, you could peek in `nrf52-code/boards/dk-solution`, which has a complete set of the required BSP changes.

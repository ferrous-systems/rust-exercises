# Handle UART Received Data

Now you have a working framework, with UART interrupts firing and being handled. Let's try adding a `#[local]` resource of our own.

✅ Modify the program so that it checks which character has been received on the UART, and have report the total number of ASCII letter `'x'` characters that have been received.

You'll probably want an additional resource for this.

See [`stm32-code/standalone-app/src/bin/standalone-rtic-uart-complete.rs`](../../../stm32-code/standalone-app/src/bin/standalone-rtic-uart-complete.rs) for a solution.

✅ Bonus exercise: merge the `standalone-rtic-blink` and `standalone-rtic-uart` programs, so that ASCII characters sent over the UART cause the colour of the blinking LED to change. You may need a `#[shared]` resource for this.

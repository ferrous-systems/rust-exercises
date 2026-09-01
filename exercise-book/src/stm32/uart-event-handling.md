# Event Handling

✅ Open the [`stm32-code/standalone-app/src/bin/standalone-rtic-event.rs`](../../../stm32-code/standalone-app/src/bin/standalone-rtic-event.rs) file.

Below the `idle` function you'll see a `#[task]` handler, a function. This *task* is bound to the `USART1` interrupt signal and will be executed, function-call style, every time the interrupt signal is raised by the hardware.

Add these two lines to the `#[init]` function:

```rust,ignore
board.usart1.configure(bsp::APB2_PERIPH_CLK_HZ);
board.usart1.rx_interrupt_enable(true);
```

This will configure UART and cause a UART interrupt to fire whenever the UART has data in its buffer.

✅ Run the modified `standalone-rtic-event` application. Then connect Serial Terminal to your board (as you did in the [Green and Yellow exercise](./green-yellow-porting.md)) and send some bytes to the UART.

Note that all tasks will be prioritized over the `idle` function so the execution of `idle` will be interrupted (paused) by the `usart1_handler` task. When the `usart1_handler` task finishes (returns) the execution of the `idle` will be resumed. This will become more obvious in the next section.

What do you observe? Will `usart1_handler` ever stop being called?

✅ Try this: add an infinite loop to the end of `init` so that it never returns. Now run the program and send some bytes to the UART. What behavior do you observe? How would you explain this behavior? (hint: look at the `rtic-expansion.rs` file: under what conditions is the `init` function executed?)

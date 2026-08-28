# Async Tasks

RTIC also supports long-lived tasks which are not tied to any particular interrupt. These must be written as *async* functions.

✅ Load the [`stm32-code/standalone-app/src/bin/standalone-rtic-blink.rs`](../../../stm32-code/standalone-app/src/bin/standalone-rtic-blink.rs) file and observe how async tasks can be created.

Why does the code say:

```rust,ignore
next_blink += BLINK_PERIOD_MS.millis();
Mono::delay_until(next_blink).await;
```

When it could have said:

```rust,ignore
Mono::delay(BLINK_PERIOD_MS.millis()).await;
```

<details>
<summary>Answer</summary>

There is a small amount of time spent in handling the interrupt, controling the LEDs and doing the logging. With a naive loop, the total time spent would be *BLINK_PERIOD_MS, plus a little bit*. This would cause the timekeeping to drift over minutes or hours.

By saying "OK, it was X, so now wait until X + BLINK_PERIOD_MS", the extra time spent on overheads is accounted for and no time slippage should occur.

</details>

✅ Run the `standalone-rtic-blink` program

Do you observe the timestamps in the log messages? Is it drifting at all?

✅ Edit the file to adjust the delays and which LED is (or which LEDs are) being blinked, and run to check your changes.


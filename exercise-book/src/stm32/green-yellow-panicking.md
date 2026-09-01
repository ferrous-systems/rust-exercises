# Panicking

✅ Open the [`stm32-code/standalone-app/src/bin/standalone-panic.rs`](../../../stm32-code/standalone-app/src/bin/standalone-panic.rs) file and click the "Run" button (or run with `cargo run --bin panic`).

This program attempts to index an array beyond its length and this results in a panic.

```console
$ cargo run --bin standalone-panic
   Compiling standalone-app v0.1.0 (/Users/jonathan/Documents/ferrous-systems/rust-exercises/stm32-code/standalone-app)
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.43s
     Running `probe-rs run --chip STM32U5A5ZJ target/thumbv8m.main-none-eabi/debug/standalone-panic`
      Erasing ✔ 100% [####################]  16.00 KiB @ 164.38 KiB/s (took 0s)
     Finished in 0.63s
Hello, this is standalone-panic!
[ERROR] PANIC: panicked at standalone-app/src/bin/standalone-panic.rs:29:13 (standalone_panic src/bin/standalone-panic.rs:41)
Firmware exited unexpectedly: Breakpoint(Unknown)
Core 0
    Frame 0: bkpt @ 0x80002d6 inline
        /Users/jonathan/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cortex-m-0.7.9/src/asm.rs:18:14
    Frame 1: panic_handler @ 0x80002d6
        /Users/jonathan/Documents/ferrous-systems/rust-exercises/stm32-code/standalone-app/src/bin/standalone-panic.rs:42:5
    Frame 2: panic_fmt @ 0x8001942
        /rustc/f46ec5218fe7829ac18323b5ee0b409a63169f27/library/core/src/panicking.rs:80:14
    Frame 3: panic_bounds_check @ 0x8001860
        /rustc/f46ec5218fe7829ac18323b5ee0b409a63169f27/library/core/src/panicking.rs:271:5
    Frame 4: __cortex_m_rt_main @ 0x8000306
        /Users/jonathan/Documents/ferrous-systems/rust-exercises/stm32-code/standalone-app/src/bin/standalone-panic.rs:29:13
    Frame 5: __cortex_m_rt_main_trampoline @ 0x800030e
        /Users/jonathan/Documents/ferrous-systems/rust-exercises/stm32-code/standalone-app/src/bin/standalone-panic.rs:18:1
    Frame 6: Reset @ 0x800027c
Error: Breakpoint(Unknown)
```

In `no_std` programs the behavior of panic is defined using the `#[panic_handler]` attribute. In the example, the *panic handler* is defined in the `standalone-panic.rs` file, but we can change it:

✅ Change `standalone-app/src/bin/standalone-panic.rs` and change the panic panic handler, like:

```rust ignore
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // Note this argument has changed -------v
    defmt::error!("PANIC: {}", defmt::Display2Format(info));
    cortex_m::asm::bkpt();
    loop {}
}
```

Now run the program again. Note the output has changed? We're now using the `core::format::Display` formatted output of a `PanicInfo`. This gives more information, but some expensive formatting has to be done on the microcontroller to generate that output. Previously, the raw `PanicInfo` object was being passed to `probe-rs` and it was doing the formatting for you. But, `probe-rs` wasn't able to produce the panic message because that message was not a fixed string but required some run-time variables before it could be formatted. Currently `defmt` and `probe-rs` cannot handle that.

Would you prefer smaller binaries or richer panic messages? How much bigger did the program get when you made this change?


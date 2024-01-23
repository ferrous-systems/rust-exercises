# Panicking

✅ Open the `nrf52-code/radio-app/src/bin/panic.rs` file and click the "Run" button (or run with `cargo run --bin panic`).

This program attempts to index an array beyond its length and this results in a panic.

```text
   Compiling radio_app v0.0.0 (/Users/jonathan/Documents/ferrous-systems/rust-exercises/nrf52-code/radio-app)
    Finished dev [optimized + debuginfo] target(s) in 0.82s
     Running `probe-rs run --chip nRF52840_xxAA target/thumbv7em-none-eabihf/debug/panic`
     Erasing sectors ✔ [00:00:00] [#####################] 8.00 KiB/8.00 KiB @ 26.69 KiB/s (eta 0s )
 Programming pages   ✔ [00:00:00] [#####################] 8.00 KiB/8.00 KiB @ 29.80 KiB/s (eta 0s )    Finished in 0.592s
ERROR panicked at src/bin/panic.rs:30:13:
index out of bounds: the len is 3 but the index is 3
└─ panic_probe::print_defmt::print @ /Users/jonathan/.cargo/registry/src/index.crates.io-6f17d22bba15001f/panic-probe-0.3.1/src/lib.rs:104
Frame 0: "HardFault handler. Cause: Escalated UsageFault (Undefined instruction)." @ 0x00001658
Frame 1: __udf @ 0x00001504 inline
       ./asm/lib.rs:48:1
Frame 2: __udf @ 0x0000000000001504
       ./asm/lib.rs:51:17
Frame 3: udf @ 0x000014f0
       /Users/jonathan/.cargo/registry/src/index.crates.io-6f17d22bba15001f/cortex-m-0.7.7/src/asm.rs:43:5
Frame 4: hard_fault @ 0x000014e2
       /Users/jonathan/.cargo/registry/src/index.crates.io-6f17d22bba15001f/panic-probe-0.3.1/src/lib.rs:86:5
Frame 5: panic @ 0x000014b0
       /Users/jonathan/.cargo/registry/src/index.crates.io-6f17d22bba15001f/panic-probe-0.3.1/src/lib.rs:54:9
Frame 6: panic_fmt @ 0x0000034a
       /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/panicking.rs:72:14
Frame 7: panic_bounds_check @ 0x000003fe
       /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/panicking.rs:190:5
Frame 8: bar @ 0x00000180
       /Users/jonathan/Documents/ferrous-systems/rust-exercises/nrf52-code/radio-app/src/bin/panic.rs:30:13
Frame 9: foo @ 0x00000176
       /Users/jonathan/Documents/ferrous-systems/rust-exercises/nrf52-code/radio-app/src/bin/panic.rs:24:2
Frame 10: __cortex_m_rt_main @ 0x000002de
       /Users/jonathan/Documents/ferrous-systems/rust-exercises/nrf52-code/radio-app/src/bin/panic.rs:13:5
Frame 11: __cortex_m_rt_main_trampoline @ 0x0000018a
       /Users/jonathan/Documents/ferrous-systems/rust-exercises/nrf52-code/radio-app/src/bin/panic.rs:9:1
Frame 12: memmove @ 0x0000013c
Frame 13: memmove @ 0x0000013c
Error: CPU halted unexpectedly.
```

In `no_std` programs the behavior of panic is defined using the `#[panic_handler]` attribute. In the example, the *panic handler* is defined in the `panic_log` crate but we can also implement it manually:

✅ Comment out the `use radio_app as _;` import and add the following function to the example:

```rust ignore
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("{}", defmt::Debug2Format(info));
    asm::udf();
}
```

Now run the program again. Try changing the format string given to `defmt::error!`.

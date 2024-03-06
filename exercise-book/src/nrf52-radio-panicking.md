# Panicking

✅ Open the [`nrf52-code/radio-app/src/bin/panic.rs`](../../nrf52-code/radio-app/src/bin/panic.rs) file and click the "Run" button (or run with `cargo run --bin panic`).

This program attempts to index an array beyond its length and this results in a panic.

```console
$ cargo run --bin panic
   Compiling defmt-macros v0.3.6
   Compiling defmt v0.3.5
   Compiling defmt-rtt v0.4.0
   Compiling panic-probe v0.3.1
   Compiling dk v0.0.0 (/Users/jonathan/Documents/ferrous-systems/rust-exercises/nrf52-code/boards/dk)
   Compiling radio_app v0.0.0 (/Users/jonathan/Documents/ferrous-systems/rust-exercises/nrf52-code/radio-app)
    Finished dev [optimized + debuginfo] target(s) in 1.27s
     Running `probe-rs run --chip nRF52840_xxAA target/thumbv7em-none-eabihf/debug/panic`
      Erasing ✔ [00:00:00] [#######################################################################################################################################] 16.00 KiB/16.00 KiB @ 32.26 KiB/s (eta 0s )
  Programming ✔ [00:00:00] [#######################################################################################################################################] 16.00 KiB/16.00 KiB @ 41.48 KiB/s (eta 0s )    Finished in 0.904s
ERROR panicked at src/bin/panic.rs:30:13:
index out of bounds: the len is 3 but the index is 3
└─ panic_probe::print_defmt::print @ /Users/jonathan/.cargo/registry/src/index.crates.io-6f17d22bba15001f/panic-probe-0.3.1/src/lib.rs:104
`dk::fail()` called; exiting ...
Frame 0: fail @ 0x00001308
       /Users/jonathan/.cargo/registry/src/index.crates.io-6f17d22bba15001f/cortex-m-semihosting-0.5.0/src/lib.rs:201:13
Frame 1: __cortex_m_rt_HardFault @ 0x000016a6 inline
       /Users/jonathan/Documents/ferrous-systems/rust-exercises/nrf52-code/radio-app/src/lib.rs:12:5
Frame 2: __cortex_m_rt_HardFault_trampoline @ 0x00000000000016a2
       /Users/jonathan/Documents/ferrous-systems/rust-exercises/nrf52-code/radio-app/src/lib.rs:10:1
Frame 3: "HardFault handler. Cause: Escalated UsageFault (Undefined instruction)." @ 0x000016a6
Frame 4: __udf @ 0x00001530 inline
       ./asm/lib.rs:48:1
Frame 5: __udf @ 0x0000000000001530
       ./asm/lib.rs:51:17
Frame 6: udf @ 0x0000151c
       /Users/jonathan/.cargo/registry/src/index.crates.io-6f17d22bba15001f/cortex-m-0.7.7/src/asm.rs:43:5
Frame 7: hard_fault @ 0x0000150e
       /Users/jonathan/.cargo/registry/src/index.crates.io-6f17d22bba15001f/panic-probe-0.3.1/src/lib.rs:86:5
Frame 8: panic @ 0x000014dc
       /Users/jonathan/.cargo/registry/src/index.crates.io-6f17d22bba15001f/panic-probe-0.3.1/src/lib.rs:54:9
Frame 9: panic_fmt @ 0x0000034a
       /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/panicking.rs:72:14
Frame 10: panic_bounds_check @ 0x000003fe
       /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/panicking.rs:190:5
Frame 11: bar @ 0x00000180
       /Users/jonathan/Documents/ferrous-systems/rust-exercises/nrf52-code/radio-app/src/bin/panic.rs:30:13
Frame 12: foo @ 0x00000176
       /Users/jonathan/Documents/ferrous-systems/rust-exercises/nrf52-code/radio-app/src/bin/panic.rs:24:2
Frame 13: __cortex_m_rt_main @ 0x000002de
       /Users/jonathan/Documents/ferrous-systems/rust-exercises/nrf52-code/radio-app/src/bin/panic.rs:13:5
Frame 14: __cortex_m_rt_main_trampoline @ 0x0000018a
       /Users/jonathan/Documents/ferrous-systems/rust-exercises/nrf52-code/radio-app/src/bin/panic.rs:9:1
Frame 15: memmove @ 0x0000013c
Frame 16: memmove @ 0x0000013c
Error: Semihosting indicates exit with failure code: 0x020023 (131107)
```

In `no_std` programs the behavior of panic is defined using the `#[panic_handler]` attribute. In the example, the *panic handler* is defined in the `panic-probe` crate but we can also implement a custom one in our binary:

✅ Change `radio-app/lib.rs` and the remove the `use panic_probe as _;` line and add a custom panic handler, like:

```rust ignore
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("Oops!! {}", defmt::Debug2Format(info));
    dk::fail();
}
```

Now run the program again. Try again, but without printing the `info` variable. Can you print `info` without `defmt::Debug2Format(..)` wrapped around it? Why not?

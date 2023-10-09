# Panicking

✅ Open the `nrf52-code/radio-app/src/bin/panic.rs` file and click the "Run" button (or run with `cargo run --bin panic`).

This program attempts to index an array beyond its length and this results in a panic.

```console
   Compiling radio v0.0.0 (/Users/jonathan/Documents/ferrous-systems/rust-exercises/nrf52-code/radio)
    Finished dev [optimized + debuginfo] target(s) in 0.92s
     Running `probe-run --chip nRF52840_xxAA target/thumbv7em-none-eabihf/debug/panic`
(HOST) INFO  flashing program (2 pages / 8.00 KiB)
(HOST) INFO  success!
────────────────────────────────────────────────────────────────────────────────
ERROR panicked at 'index out of bounds: the len is 3 but the index is 3', src/bin/panic.rs:30:13
└─ panic_probe::print_defmt::print @ /Users/jonathan/.cargo/registry/src/index.crates.io-6f17d22bba15001f/panic-probe-0.3.1/src/lib.rs:104
────────────────────────────────────────────────────────────────────────────────
(HOST) INFO  program has used at least 0.80/254.93 KiB (0.3%) of stack space
stack backtrace:
   0: HardFaultTrampoline
      <exception entry>
   1: lib::inline::__udf
        at ./asm/inline.rs:181:5
   2: __udf
        at ./asm/lib.rs:51:17
   3: cortex_m::asm::udf
        at /Users/jonathan/.cargo/registry/src/index.crates.io-6f17d22bba15001f/cortex-m-0.7.7/src/asm.rs:43:5
   4: panic_probe::hard_fault
        at /Users/jonathan/.cargo/registry/src/index.crates.io-6f17d22bba15001f/panic-probe-0.3.1/src/lib.rs:86:5
   5: rust_begin_unwind
        at /Users/jonathan/.cargo/registry/src/index.crates.io-6f17d22bba15001f/panic-probe-0.3.1/src/lib.rs:54:9
   6: core::panicking::panic_fmt
        at /rustc/5680fa18feaa87f3ff04063800aec256c3d4b4be/library/core/src/panicking.rs:67:14
   7: core::panicking::panic_bounds_check
        at /rustc/5680fa18feaa87f3ff04063800aec256c3d4b4be/library/core/src/panicking.rs:162:5
   8: panic::bar
        at src/bin/panic.rs:30:13
   9: panic::foo
        at src/bin/panic.rs:24:2
  10: panic::__cortex_m_rt_main
        at src/bin/panic.rs:13:5
  11: main
        at src/bin/panic.rs:9:1
  12: Reset
(HOST) ERROR the program panicked
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

Now run the program again. Try changing the format string of the `panic!` macro.

# Panicking

✅ Open the [`nrf52-code/radio-app/src/bin/panic.rs`](../../nrf52-code/radio-app/src/bin/panic.rs) file and click the "Run" button (or run with `cargo run --bin panic`).

This program attempts to index an array beyond its length and this results in a panic.

```console
$ cargo run --bin panic
   Compiling radio_app v0.0.0 (/Users/jonathan/Documents/ferrous-systems/rust-exercises/nrf52-code/radio-app)
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.30s
     Running `probe-rs run --chip=nRF52840_xxAA --allow-erase-all --log-format=oneline target/thumbv7em-none-eabihf/debug/panic`
      Erasing ✔ 100% [####################]   8.00 KiB @  15.72 KiB/s (took 1s)
  Programming ✔ 100% [####################]   8.00 KiB @  12.85 KiB/s (took 1s)                                                                                                                                                                                                                                   Finished in 1.13s
00:00:00.000000 [ERROR] panicked at src/bin/panic.rs:30:13:
index out of bounds: the len is 3 but the index is 3 (panic_probe panic-probe-0.3.2/src/lib.rs:104)
`dk::fail()` called; exiting ...
Frame 0: syscall1 @ 0x000012ec inline
       /Users/jonathan/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cortex-m-semihosting-0.5.0/src/lib.rs:201:13
Frame 1: report_exception @ 0x00000000000012ea inline
       /Users/jonathan/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cortex-m-semihosting-0.5.0/src/macros.rs:28:9
Frame 2: exit @ 0x00000000000012ea inline
       /Users/jonathan/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cortex-m-semihosting-0.5.0/src/debug.rs:74:25
Frame 3: fail @ 0x00000000000012ea
       /Users/jonathan/Documents/ferrous-systems/rust-exercises/nrf52-code/boards/dk/src/lib.rs:456:9
Frame 4: <unknown function @ 0x000016e2> @ 0x000016e2
Error: Semihosting indicated exit with reason: 0x20023
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

# Panicking

✅ Open the [`nrf52-code/radio-app/src/bin/panic.rs`](../../nrf52-code/radio-app/src/bin/panic.rs) file and click the "Run" button (or run with `cargo run --bin panic`).

This program attempts to index an array beyond its length and this results in a panic.

```console
$ cargo run --bin panic
   Compiling radio_app v0.0.0 (/Users/jonathan/Documents/ferrous-systems/rust-exercises/nrf52-code/radio-app)
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.03s
     Running `probe-rs run --chip=nRF52840_xxAA --allow-erase-all --log-format=oneline target/thumbv7em-none-eabihf/debug/panic`
      Erasing ✔ 100% [####################]  12.00 KiB @  18.71 KiB/s (took 1s)
  Programming ✔ 100% [####################]  12.00 KiB @  14.22 KiB/s (took 1s)                                                                                                                                                                                                                                   Finished in 1.49s
00:00:00.000000 [ERROR] panicked at src/bin/panic.rs:30:13:
index out of bounds: the len is 3 but the index is 3 (radio_app src/lib.rs:8)
`dk::fail()` called; exiting ...
Frame 0: syscall1 @ 0x00000cac inline
       /Users/jonathan/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cortex-m-semihosting-0.5.0/src/lib.rs:201:13
Frame 1: report_exception @ 0x0000000000000caa inline
       /Users/jonathan/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cortex-m-semihosting-0.5.0/src/macros.rs:28:9
Frame 2: exit @ 0x0000000000000caa
       /Users/jonathan/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cortex-m-semihosting-0.5.0/src/debug.rs:74:25
Frame 3: fail @ 0x0000043e
       /Users/jonathan/Documents/ferrous-systems/rust-exercises/nrf52-code/boards/dk/src/lib.rs:456:9
Frame 4: <unknown function @ 0x000c0a36> @ 0x000c0a36
```

In `no_std` programs the behavior of panic is defined using the `#[panic_handler]` attribute. In the example, the *panic handler* is defined in the `radio-app/lib.rs` file, but we can change it:

✅ Change `radio-app/lib.rs` and change the panic panic handler, like:

```rust ignore
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("Oops!! {}", defmt::Debug2Format(info));
    dk::fail();
}
```

Now run the program again. Try again, but without printing the `info` variable. Can you print `info` without `defmt::Debug2Format(..)` wrapped around it? Why not?

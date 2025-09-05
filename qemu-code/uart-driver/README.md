# Rust UART Driver exercise

This folder contains a small Rust no-std application, which is designed to run
inside a QEMU emulation of an Armv8-R Cortex-R52 system. We build the code
using the `armv8r-none-eabihf` target.

The application talks to the outside world through a UART driver. We have
provided two - a working one, and a template one that doesn't work which you
need to fix.

## Prerequisites

This demo is designed to run with Ferrocene, which ships the
`armv8r-none-eabihf` target.

### Ferrrocene

Run:

```bash
criticalup install
criticalup link create
cargo run
```

We use `criticalup link create` to teach `rustup` that `+ferrocene` means 'use criticalup'. We then have `rust-toolchain.toml` file that tells `rustup` that we want to use `+ferrocene` by default.

### Rust

If you want to run it with the upstream Rust compiler, you will need to use
`nightly`, and tell `cargo` to build the standard library from source:

```bash
cargo +nightly run -Zbuild-std=core
```

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](../LICENSE-MIT) or
<http://opensource.org/licenses/MIT>) at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

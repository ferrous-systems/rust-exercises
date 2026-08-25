# STM32 TrustZone exercises

This folder contains source code for our TrustZone exercises.

This code is designed to run on an ST Micro NUCLEO-U5A5ZJ-Q board, featuring an
STM32U5A5ZJ microcontroller.

There are five packages in this directory.

* [`step1-option-bytes`](./step1-option-bytes/) - a Nonsecure State binary that
  sets the Option Bytes so that the board boots into Secure State from then on
  (i.e. sets TZEN=1)
* [`step2-secure-watermark`](./step2-secure-watermark/) - a Secure State binary
  that sets the Option Bytes so that the board allows Nonsecure State execution
  from Flash Bank 2
* [`nucleo-u5a5zj`](./nucleo-u5a5zj/) - a small BSP for the NUCLEO-U5A5ZJ-Q
  board
* [`stm32u5-tiny-hal`](./stm32u5-tiny-hal/) - a small HAL for STM32U5 series
  MCUs
* [`secure-loader`](./secure-loader) - a Secure State binary which initialises
  hardware and then boots a Nonsecure State binary in the other half of flash
* [`nonsecure-app`](./nonsecure-app) - a fairly normal Rust Embedded program
  designed to be bootstrapped by the `secure-loader`
* [`standalone-app`](./standalone-app/) - a fairly normal Rust Embedded program
  designed to boot without `secure-loader` being present

For TrustZone work, you will need to load both the `secure-loader` binary and
the `nonsecure-app` binary onto your board. The `secure-loader` will run first,
in Secure State, and it will then start the `nonsecure-app` in Nonsecure State.
The `nonsecure-app` has access to services exported from the `secure-loader`
which it can call at will.

Because this project uses the [unstable `cmse_nonsecure_entry` feature], you must
use Nightly Rust to build it.

[unstable `cmse_nonsecure_entry` feature]: https://doc.rust-lang.org/nightly/unstable-book/language-features/cmse-nonsecure-entry.html


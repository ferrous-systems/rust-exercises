# STM32 TrustZone exercises

This folder contains source code for our TrustZone exercises.

This code is designed to run on an ST Micro NUCLEO-U5A5ZJ-Q board, featuring an
STM32U5A5ZJ microcontroller.

There are five packages in this directory.

* [`option-bytes`](./option-bytes/) - a non-secure app that sets the Option Bytes
  so that the board boots into Secure Mode from them on (i.e. sets TZEN=1)
* [`nucleo-u5a5zj`](./nucleo-u5a5zj/) - a small BSP for the NUCLEO-U5A5ZJ-Q
  board
* [`stm32u5-tiny-hal`](./stm32u5-tiny-hal/) - a small HAL for STM32U5 series
  MCUs
* [`secure-loader`](./secure-loader) - a secure app which initialises hardware
  and then boots a nonsecure app in the other half of flash
* [`nonsecure-app`](./nonsecure-app) - a fairly normal Rust Embedded program

You will need to load both the `secure-loader` binary and the `nonsecure-app` binary
onto your board. The `secure-loader` will run first, in Secure Mode, and it will
then start the `nonsecure-app` in Nonsecure Mode. The `nonsecure-app` has access
to services exported from the `secure-loader` which it can call at will.

Because this project uses the [unstable `cmse_nonsecure_entry` feature], you must
use Nightly Rust to build it.

[unstable `cmse_nonsecure_entry` feature]: https://doc.rust-lang.org/nightly/unstable-book/language-features/cmse-nonsecure-entry.html


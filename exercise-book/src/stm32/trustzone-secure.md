# Booting in Secure Mode

## Introduction

The STM32U5A5ZJ-Q has 4 MiB of Flash, which is available via two address ranges.
We'll see this is fairly common for TrustZone peripherals - one address range is
for *nonsecure access* and one is for *secure access*.

- The nonsecure address range for Flash is `0x0800_0000..0x0803_FFFF`.
- The secure address range for Flash is `0x0C00_0000..0x0C03_FFFF`.

You see the same contents regardless of which address range you use. But by
using the secure address range you gain additional permissions. This is thanks
to a feature of TrustZone-M called the [*Implementation Defined Attribution
Unit*][attr-unit] - basically ST have decided that the implementation defined
portion of the permissions comes from the memory address. 

[attr-unit]:
    https://developer.arm.com/documentation/100690/0201/Attribution-units--SAU-and-IDAU-

Because our STM32U5A5ZJ-Q has `TZEN=1` set in the Option Bytes (see [STM32
Introduction and Preparation]), it will boot from the address given in the
`FLASH_SECBOOTADD0R` register (which is loaded from the Option Bytes). This
defaults to `0x0C00_0000`.

[STM32 Introduction and Preparation]: ./preparation.md#option-bytes

The other half of your permissions comes from a Cortex-M33 peripheral called the
[*Secure Atribution Unit*][attr-unit] and there we can mark particular parts of
our 32-bit (4 GiB) address space as being either:

- *Secure* (S)
- *Nonsecure* (NS), or
- *Nonsecure Callable* (NSC).

That last one means that the code will run in *Secure mode* but it is offered as
an API to code running in *Nonsecure mode*. For example, if *Secure mode* was
looking after your signing keys, you might have a *Nonsecure Callable* API for
"Please verify this digital signature". The signature verification process needs
the keys but it is very important the keys themselves do not leak out into
nonsecure world and hence into the hands of a potential attacker.

You need both IDPAU and SAU to say yes in order to make a secure access to
Flash, RAM or a memory-mapped peripheral. This, and many more details about
TrustZone-M are available from Arm in their document [TrustZone(R) technology
for Armv8-M Architecture][tz-arch].

[tz-arch]: https://developer.arm.com/documentation/100690

## Running the Template

This is all fairly complicated, so we've set up a template Secure-mode
application to get you started. Let's run it.

```console
$ cargo run --bin secure-loader
   Compiling secure-loader v0.1.0 (/Users/jonathan/Documents/ferrous-systems/rust-exercises/stm32-code/secure-loader)
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.16s
     Running `probe-rs run --chip STM32U5A5ZJ target/thumbv8m.main-none-eabi/debug/secure-loader`
      Erasing ✔ 100% [####################]  16.00 KiB @ 167.28 KiB/s (took 0s)
     Finished in 0.51s
Hello, this is secure-loader. Configuring peripherals...
...LEDs configured
The rest of this program is missing! You need to write it :)
```

The program will now hang here, and you can press `Ctrl + C` to exit `probe-rs`.

Take a look through the [`src/main.rs`] file. Important things to note include:

[`src/main.rs`]: ../../../stm32-code/secure-loader/src/main.rs

* We're using the [`cortex-m-semihosting`] crate for text output. This is pretty
  slow, but we don't need to print that much. `probe-rs` understands semihosting
  and just prints out the text.
* There's a Board Support Package (in the [`nucleo-u5a5zj-bsp` folder]), which
  will give you a useful set of drivers which you'll need to complete the
  exercise.
* We have given you a Panic Handler, and a Secure Fault handler, which will
  hopefuly give you some clues if your program goes wrong.

[`cortex-m-semihosting`]: https://crates.io/crates/cortex-m-semihosting
[`nucleo-u5a5zj-bsp` folder]: ../../../stm32-code/nucleo-u5a5zj-bsp/

If you look in [`memory.x`] you'll note that we're only telling the linker about
the first 2 MiB of Flash, and the first 768K of SRAM (which is a bank called
SRAM1). The other half of Flash, and the other SRAM banks, we are going to
reserve for the nonsecure application.

[`memory.x`]: ../../../stm32-code/secure-loader/memory.x

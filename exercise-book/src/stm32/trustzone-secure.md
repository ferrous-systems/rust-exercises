# Booting in Secure State

## Introduction

As noted before, code running in Nonsecure state **cannot** access anything
marked as *Secure* access only. But how do we mark which things should be
accessible from each state?

TrustZone does this using a combination of two *Attribution Units*:

* the [*Implementation Defined Attribution Unit*][attr-unit] (IDAU)
* the [*Secure Atribution Unit*][attr-unit]

[attr-unit]:
    https://developer.arm.com/documentation/100690/0201/Attribution-units--SAU-and-IDAU-

You need both the IDAU and the SAU to say yes in order to make a secure access
to a region of memory (be that Flash, RAM or a memory-mapped peripheral). I
think of it like the [Swiss cheese model] - giving Nonsecure state is risky, and
so you only get that access where both slices (the IDAU and SAU) have
overlapping holes. If the IDAU holes and SAU holes don't overlap then there is
no hole and Nonsecure state gets no access.

[Swiss cheese model]: https://en.wikipedia.org/wiki/Swiss_cheese_model

When ST implemented the IDAU on the STM32U5, they decided to use the memory map
to control security. That is, some memory regions are considered *Secure* and
some are considered *Nonsecure*, and this is fixed by the silicon at design
time. You cannot program the ST's IDAU - it is what it is.

Other manufacturers may make different choices when implementing their IDAU.

Some parts of the STM32U5 appear in the memory map *twice*. For example, the
STM32U5A5ZJ-Q has 4 MiB of Flash, which is available via two address ranges. One
address range is for *nonsecure access* and one is for *secure access*.

- The nonsecure address range for Flash is `0x0800_0000..0x0803_FFFF`.
- The secure address range for Flash is `0x0C00_0000..0x0C03_FFFF`.

You see the same contents regardless of which address range you use, but only
code running in *Secure state* can use the secure address range.

🔎 Note that you must execute code from the correct address range, because Arm
machine code generally makes assumptions about the address is it located at, and
it won't work if you try to execute it from some other memory address range. So
our `memory.x` files will need to pick the correct memory range out of the two
above, depending on what Security state the program will be started in.

Permissions from the IDAU are fairly broad - it is fixed in silicon, and offers
no run-time control, nor any way of subdividing these large blocks of memory.
That is why Arm give us a second layer of permissions - the SAU.

The SAU is a memory-mapped peripheral within in the Cortex-M33 processor, and we
can use it to mark particular parts of our 32-bit (4 GiB) address space as being
either:

- *Secure* (S)
- *Nonsecure* (NS), or
- *Nonsecure Callable* (NSC).

That last one means that the code will run in *Secure State* but it is offered
as an API to code running in *Nonsecure State*. For example, if *Secure State*
was looking after your signing keys, you might have a *Nonsecure Callable* API
for "Please verify this digital signature". The signature verification process
needs the keys but it is very important the keys themselves do not leak out into
Nonsecure State and hence into the hands of a potential attacker.

Many more details about TrustZone-M are available from Arm in their document
[TrustZone(R) technology for Armv8-M Architecture][tz-arch].

[tz-arch]: https://developer.arm.com/documentation/100690

## Running the Template

Because our STM32U5A5ZJ-Q has `TZEN=1` set in the Option Bytes (see [STM32
Introduction and Preparation](./preparation.md#option-bytes)), it will boot from
the address given in the `FLASH_SECBOOTADD0R` register (which is loaded from the
Option Bytes). This defaults to `0x0C00_0000`.

This is all fairly complicated, so we've set up a template Secure State
application to get you started, called `secure-loader`. Let's run it:

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
reserve for an application running in Nonsecure State.

[`memory.x`]: ../../../stm32-code/secure-loader/memory.x

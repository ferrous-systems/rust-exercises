# Changing to Nonsecure Mode

We got secure-mode running in the previous chapter. But we also want to run a
nonsecure application.

Your tasks are to:

* Call the BSP function to program the SAU.
* Call the BSP function to program the Global TrustZone Controller's Memory
  Protection Controller for SRAM3
* Use the `cortex-m` library function to bootstrap the Nonsecure application

Once we've done that, we can load the `nonsecure-app` and hopefully the
`secure-loader` will boot and then jump to the `nonsecure-app`.

The `nonsecure-app` sees the world like this:

| Description  | Start Address | End Address   | IDPAU     | SAU                 | Contains       |
|--------------|---------------|---------------|-----------|---------------------|----------------|
| Flash Bank 1 | `0x0800_0000` | `0x081F_FFFF` | Nonsecure | ---                 | *Unused*       |
| Flash Bank 2 | `0x0820_0000` | `0x083F_FFFF` | Nonsecure | Region 0, Nonsecure | Code           |
| SRAM1        | `0x2000_0000` | `0x200B_FFFF` | Nonsecure | ---                 | *Unused*       |
| SRAM3        | `0x200D_0000` | `0x201A_FFFF` | Nonsecure | Region 1, Nonsecure | Data and Stack |
| Peripherals  | `0x4000_0000` | `0x4FFF_FFFF` | Nonsecure | Region 2, Nonsecure | Peripherals    |

The `secure-loader` sees the world like this:

| Description  | Start Address | End Address   | IDPAU     | SAU                 | Contains           |
|--------------|---------------|---------------|-----------|---------------------|--------------------|
| Flash Bank 1 | `0x0800_0000` | `0x081F_FFFF` | Nonsecure | ---                 | *Unused*           |
| Flash Bank 2 | `0x0820_0000` | `0x083F_FFFF` | Nonsecure | Region 0, Nonsecure | Code               |
| Flash Bank 1 | `0x0C00_0000` | `0x0C1F_FFFF` | Secure    | ---                 | Secure loader code |
| Flash Bank 2 | `0x0C20_0000` | `0x0C3F_FFFF` | Secure    | ---                 | *Unused alias*     |
| SRAM1        | `0x2000_0000` | `0x200B_FFFF` | Nonsecure | ---                 | *Unused*           |
| SRAM3        | `0x200D_0000` | `0x201A_FFFF` | Nonsecure | Region 1, Nonsecure | Data and Stack     |
| SRAM1        | `0x3000_0000` | `0x300B_FFFF` | Secure    | ---                 | *Unused alias*     |
| SRAM3        | `0x300D_0000` | `0x301A_FFFF` | Secure    | ---                 | *Unused alias*     |
| Peripherals  | `0x4000_0000` | `0x4FFF_FFFF` | Nonsecure | Region 2, Nonsecure | Peripherals        |
| Peripherals  | `0x5000_0000` | `0x5FFF_FFFF` | Secure    | ---                 | Secure Peripherals |

## Task 1 - Program the SAU

By default the SAU will say no to anything the Nonsecure mode asks for (which is
a very reasonable default). However, our `nonsecure-app` is going to need *some*
resources, so we'll give it:

* All of Flash Bank 2
* All of SRAM3
* All of the Peripherals at `0x4000_0000`

To do this, the `cortex-m` library has a SAU driver we can use. And the BSP has
helpfully given us access to an instance of the SAU driver, as `bsp.sau`.

We need to call the init function, and pass in a slice of `SauRegion` objects.

```rust,ignore
use cortex_m::peripheral::sau::{SauRegion, SauRegionAttribute};
let region0 = SauRegion {
    base_address: 0x0820_0000
    limit_address: 0x083F_FFFF,
    attribute: SauRegionAttribute::NonSecure,
}
```

Set up the three regions we need, and pass them in a slice to the
`bsp.sau.init()` function.

<details>
<summary>Hint</summary>

There's a function in the BSP that will do this for you.

</details>

### Task 2 - Program the Global TrustZone Controller's Memory Protection Controller

The STM32U5 has a separate device for controlling which parts of which SRAM are
available to nonsecure mode. This device can apportion memory on a 512-byte
block by block basis, which seems overkill. We just want to leave SRAM1 for
Secure mode, and give all of SRAM3 to Nonsecure mode. As SRAM3 is 832 KiB in
length, that's 1664 blocks we have to individual flip over to Nonsecure mode.

The GTZC MPCBB peripheral does this with a whole bunch of 32-bit registers,
where each register controls 32 blocks (1 bit per block). SRAM3 therefore has 52
registers we need to hit.

Luckily for you, I wrote a driver for this, you can you just call
`bsp.gztc.map_addresses_nonsecure` and give it the address range for SRAM3. It
will then hit all the right bits in all the right registers for you.

You will also want to call `bsp.gztc.allow_secure_read_write` to tell it that
although SRAM3 is given over to Nonsecure mode, Secure mode would still like to
read it.

<details>
<summary>Hint</summary>

There's a function in the BSP that will do this for you.

</details>

## Task 3 - Bootstrap the Nonsecure world

Nonsecure World and Secure World have their own copies of some important things:

* They have their own Nested Vector Interrupt Controller (NVIC), which is a
  memory-mapped peripheral at `0xE000_E100`
* They have their own System Control Block (SCB), which is a memory-mapped
  peripheral at `0xE000_ED04`
* They have their own Main Stack Pointers, which is a system register called
  `MSP`, or just `SP`

These things have the same name regardless of which world you are in - `MSP` is
the Secure Main Stack Pointer when you are Secure Mode, and is the Nonsecure
Main Stack Pointer when you are in Nonsecure Mode.

But, Secure mode also gets access to the Nonsecure mode values as well, through
different names.

* The Nonsecure MSP is available as system register called `MSP_NS` 
* The Nonsecure NVIC is available at `0xE002_E100`
* The Nonsecure SCB is available at `0xE002_ED04`

Think for a moment about whether this should work the other way around - should
Nonsecure mode have access to the Secure mode peripherals?

<details>
<summary>Answer</summary>

No!

The point of TrustZone is to keep things in Secure mode (like encryption keys) a
secret from Nonsecure mode (which might get hacked when processing untrusted
input).

Secure mode has is more trusted, and has more permissions that Nonsecure mode.

</details>

To actually enter Nonsecure world, we need to:

* Set Nonsecure mode's Vector Table Offset Register to where the code lives in
  flash, so that interrupts and exceptions work
* Set Nonsecure mode's Main Stack Pointer, so it has a stack to use
* Zero all the CPU registers so we don't leak any secrets to Nonsecure mode
* Execute the `BXNS` instruction, with the address of the Nonsecure reset
  function

Handily for us, this has been added to the `cortex-m` library. All you need to
do is:

```rust,ignore
let ns_app_base = ....; // some number goes here
unsafe {
    cortex_m::asm::bootload_ns(ns_app_base as *const u32, bsp.scb_ns);
}
```

As an aside, we're using a fork of the `cortex-m` crate that adds access to some
of these Nonsecure peripherals. So don't expect the documentation at
`https://docs.rs/cortex-m` to be that helpful here. You'll need to look at
<https://github.com/rust-embedded/cortex-m/tree/jp/release-cm-and-cmrt/cortex-m>
for the time being.

The `bootload_ns` function needs access to the Nonsecure System Control Base
peripheral, but the BSP has provided that so we can just and it over. Feel free
to "Go to Definition" and dig into what this function is doing.

The `bootload_ns` function is also unsafe - why is that? What rules does the
human reviewer need to check before we use that function?

Bonus question (hard-mode): Can you find a way that information might be leaked
from Secure mode to Nonsecure mode through this function? I hope not, but maybe
there's a bug!

## Some nonsecure firmware

If you run your program now, you probably end up in the Secure Fault handler.

```text
$ cargo run --bin secure-loader
   Compiling nucleo-u5a5zj-bsp v0.1.0 (/Users/jonathan/Documents/ferrous-systems/rust-exercises/stm32-code/nucleo-u5a5zj-bsp)
   Compiling secure-loader v0.1.0 (/Users/jonathan/Documents/ferrous-systems/rust-exercises/stm32-code/secure-loader)
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.42s
     Running `probe-rs run --chip STM32U5A5ZJ target/thumbv8m.main-none-eabi/debug/secure-loader`
      Erasing ✔ 100% [####################]  24.00 KiB @ 179.30 KiB/s (took 0s)
  Programming ✔ 100% [####################]  14.00 KiB @  27.27 KiB/s (took 1s)
  Finished in 0.75s
Hello, this is secure-loader. Configuring peripherals...
...SAU configured
...GTZC1 configured
...LEDs configured
Booting NS application at 0x08200000...
SECURE FAULT:
- SFAR = fffffff0
- Invalid Entry Point
- Attribution Unit Violation
```

The `fffffff0` value is suspicious - it's like `0xFFFF_FFFF` rounded down, and
`0xFFFF_FFFF` is what blank flash reads as.

That's because we haven't loaded a non-secure application! Let's do that.

```console
$ cargo run --bin nonsecure-app
   Compiling nonsecure-app v0.1.0 (/Users/jonathan/Documents/ferrous-systems/rust-exercises/stm32-code/nonsecure-app)
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.15s
     Running `probe-rs run --chip STM32U5A5ZJ target/thumbv8m.main-none-eabi/debug/nonsecure-app`
      Erasing ✔ 100% [####################]  16.00 KiB @ 143.76 KiB/s (took 0s)
  Programming ✔ 100% [####################]   9.00 KiB @  25.61 KiB/s (took 0s)
  Finished in 0.57s
Hello, this is secure-loader. Configuring peripherals...
...SAU configured
...GTZC1 configured
...LEDs configured
Booting NS application at 0x08200000...
[INFO ] Hello, this is nonsecure-app!
```

Note how even though we just loaded `nonsecure-app`, the `secure-loader` program
still ran first. But it bounced us into nonsecure mode and our nonsecure
application is happily running.

Well done!

As an aside, our nonsecure app is using `defmt` over SEGGER RTT for the text
output, which is much faster than semihosting. The trade-off is that if you
re-run the `secure-loader` program to get a new version on to the board, you
will not see any defmt output from the nonsecure side. This is because
`probe-rs` can only show you defmt output from the *program you are loading
right now* and not some random program you loaded half an hour ago. If you are
running `secure-loader`, you cannot see defmt logs from `nonsecure-app`. That's
the price you pay for defmt being so fast.

It's also why `secure-loader` is set to use the much slower semihosting output,
so we can always see it regardless of which app we've just flashed.

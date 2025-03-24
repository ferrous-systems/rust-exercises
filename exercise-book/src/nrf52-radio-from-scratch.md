# Starting a Project from Scratch

So far we have been using a pre-made Cargo project to work with the nRF52840 DK. In this section we'll see how to create a new embedded project for any microcontroller.

## Identify the microcontroller

The first step is to identify the microcontroller you'll be working with. The information about the microcontroller you'll need is:

### 1. Its processor architecture and sub-architecture

This information should be in the device's data sheet or manual. In the case of the nRF52840, the processor is an ARM Cortex-M4 core. With this information you'll need to select a compatible *compilation target*. `rustup target list` will show all the supported compilation targets.

```console
$ rustup target list
(..)
thumbv6m-none-eabi
thumbv7em-none-eabi
thumbv7em-none-eabihf
thumbv7m-none-eabi
thumbv8m.base-none-eabi
thumbv8m.main-none-eabi
thumbv8m.main-none-eabihf
```

The compilation targets will usually be named using the following format: `$ARCHITECTURE-$VENDOR-$OS-$ABI`, where the `$VENDOR` field is sometimes omitted. Bare metal and `no_std` targets, like microcontrollers, will often use `none` for the `$OS` field. When the `$ABI` field ends in `hf` it indicates that the output ELF uses the *hardfloat* Application Binary Interface (ABI).

The `thumb` targets listed above are all the currently supported ARM Cortex-M targets. The table below shows the mapping between compilation targets and ARM Cortex-M processors.

| Compilation target          | Processor                          |
| --------------------------- | ---------------------------------- |
| `thumbv6m-none-eabi`        | ARM Cortex-M0, ARM Cortex-M0+      |
| `thumbv7m-none-eabi`        | ARM Cortex-M3                      |
| `thumbv7em-none-eabi`       | ARM Cortex-M4, ARM Cortex-M7       |
| `thumbv7em-none-eabihf`     | ARM Cortex-M4*F*, ARM Cortex-M7*F* |
| `thumbv8m.base-none-eabi`   | ARM Cortex-M23                     |
| `thumbv8m.main-none-eabi`   | ARM Cortex-M33, ARM Cortex-M35P    |
| `thumbv8m.main-none-eabihf` | ARM Cortex-M33F, ARM Cortex-M35PF  |

The ARM Cortex-M ISA is backwards compatible so for example you could compile a program using the `thumbv6m-none-eabi` target and run it on an ARM Cortex-M4 microcontroller. This will work but using the `thumbv7em-none-eabi` results in better performance (ARMv7-M instructions will be emitted by the compiler) so it should be preferred. The older ISAs may also be limited in terms of the maximum number of interrupts you can define, which maybe be fewer than your newer microcontroller actually has.

### 2. Its memory layout

In particular, you need to identify how much Flash and RAM memory the device has and at which address the memory is exposed. You'll find this information in the device's data sheet or reference manual.

In the case of the nRF52840, this information is in section 4.2 (Figure 2) of its [Product Specification](https://docs.nordicsemi.com/bundle/ps_nrf52840/page/keyfeatures_html5.html).
It has:

- 1 MB of Flash that spans the address range: `0x0000_0000` - `0x0010_0000`.
- 256 KB of RAM that spans the address range: `0x2000_0000` - `0x2004_0000`.

## The `cortex-m-quickstart` project template

With all this information you'll be able to build programs for the target device. The [`cortex-m-quickstart`] project template provides the most frictionless way to start a new project for the ARM Cortex-M architecture -- for other architectures check out other project templates by the [rust-embedded] organization.

[`cortex-m-quickstart`]: https://github.com/rust-embedded/cortex-m-quickstart
[rust-embedded]: https://github.com/rust-embedded/

The recommended way to use the quickstart template is through the [`cargo-generate`] tool:

[`cargo-generate`]: https://crates.io/crates/cargo-generate

```console
cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart
```

But it may be difficult to install the `cargo-generate` tool on Windows due to its `libgit2` (C library) dependency. Another option is to download a snapshot of the quickstart template from GitHub and then fill in the placeholders in `Cargo.toml` of the snapshot.

Once you have instantiated a project using the template you'll need to fill in the device-specific information you collected in the two previous steps:

### 1. Change the default compilation target in `.cargo/config`

```toml
[build]
target = "thumbv7em-none-eabi"
```

For the nRF52840 you can choose either `thumbv7em-none-eabi` or `thumbv7em-none-eabihf`. If you are going to use the FPU then select the `hf` variant.

### 2. Enter the memory layout of the chip in `memory.x`

```text
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00000000, LENGTH = 1M
  RAM : ORIGIN = 0x20000000, LENGTH = 256K
}
```

### 3. `cargo build` now will cross compile programs for your target device

If there's no template or signs of support for a particular architecture under the rust-embedded organization then you can follow the [embedonomicon] to bootstrap support for the new architecture by yourself.

[embedonomicon]:https://docs.rust-embedded.org/embedonomicon/

## Flashing the program

To flash the program on the target device you'll need to identify the on-board debugger, if the development board has one. Or choose an external debugger, if the development board exposes a JTAG or SWD interface via some connector.

If the hardware debugger is supported by the `probe-rs` project -- for example J-Link, ST-Link or CMSIS-DAP -- then you'll be able to use `probe-rs`-based tools like `probe-rs` and `cargo-embed`. This is the case of the nRF52840 DK: it has an on-board J-Link probe.

If the debugger is not supported by `probe-rs` then you'll need to use [OpenOCD] or vendor provided software to flash programs on the board.

[OpenOCD]: http://openocd.org/

If the board does not expose a JTAG, SWD or similar interface then the microcontroller probably comes with a bootloader as part of its stock firmware. In that case you'll need to use `dfu-util` or a vendor specific tool like [`nrfdfu`](https://crates.io/crates/nrfdfu) or [`nrfutil`](https://infocenter.nordicsemi.com/index.jsp?topic=%2Fug_nrfutil%2FUG%2Fnrfutil%2Fnrfutil_intro.html) to flash programs onto the chip. This is the case of the nRF52840 Dongle.

## Getting output

If you are using one of the probes supported by `probe-rs` then you can use the [`rtt-target`] library to get text output on `cargo-embed`. The logging functionality we used in the examples is implemented using the `rtt-target` crate.

[`rtt-target`]: https://crates.io/crates/rtt-target

If that's not the case or there's no debugger on board then you'll need to add a HAL before you can get text output from the board.

## Adding a Hardware Abstraction Layer (HAL)

Now you can hopefully run programs and get output from them. To use the hardware features of the device you'll need to add a HAL to your list of dependencies. [crates.io], [lib.rs] and [awesome embedded Rust] are good places to search for HALs.

[crates.io]: https://crates.io/search?q=hal
[lib.rs]: https://lib.rs/search?q=hal
[awesome embedded Rust]: https://github.com/rust-embedded/awesome-embedded-rust#hal-implementation-crates

After you find a HAL you'll want to get familiar with its API through its [API docs] and [examples]. HAL do not always expose the exact same API, specially when it comes to initialization and configuration of peripherals. However, most HAL will implement the [`embedded-hal`] traits. These traits allow inter-operation between the HAL and [*driver* crates][drivers]. These driver crates provide functionality to interface external devices like sensors, actuators and radios over interfaces like I2C and SPI.

[API docs]: https://docs.rs/nrf52840-hal/0.10.0/nrf52840_hal/
[examples]: https://github.com/nrf-rs/nrf-hal/tree/master/examples
[`embedded-hal`]: https://crates.io/crates/embedded-hal
[drivers]: https://github.com/rust-embedded/awesome-embedded-rust#driver-crates

If no HAL is available for your device then you'll need to build one yourself. This is usually done by first generating a Peripheral Access Crate (PAC) from a [System View Description][SVD] (SVD) file using the [`svd2rust`] tool. The PAC exposes a low level, but type safe, API to modify the registers on the device. Once you have a PAC you can use of the many HALs on crates.io as a reference; most of them are implemented on top of `svd2rust`-generated PACs.

[SVD]: http://www.keil.com/pack/doc/CMSIS/SVD/html/index.html
[`svd2rust`]: https://crates.io/crates/svd2rust

## Hello, 💡

Now that you've set up your own project from scratch, you could start playing around with it by turning on one of the DK's on-board LEDs using only the HAL. Some hints that might be helpful there:

- The [Nordic Infocenter][infocenter] tells you which LED is connected to which pin.

[infocenter]: https://infocenter.nordicsemi.com/index.jsp?topic=%2Fug_nrf52840_dk%2FUG%2Fnrf52840_DK%2Fhw_buttons_leds.html

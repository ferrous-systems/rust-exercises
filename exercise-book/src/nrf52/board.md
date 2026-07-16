# Board preparation

This is the nRF52840 Development Kit (DK) board.

The board has two USB ports, J2 and J3, and an on-board J-Link programmer / debugger. USB port J3 is
the nRF52840's USB port. Connect the Development Kit to your computer using the **J2** port.
You can also refer to the image below to see the location of the different components on the board.

The development board actually has two chips. One is the nRF52840 target chip,
and the other contains a special firmware which transforms it into a SEGGER J-Link debugger probe.
Your computer will communicate via USB with the J-Link, which will in turn use the SWD protocol to interface
with the target chip. All of this avoids the need for an external probe.

💬 These directions assume you are holding the board "horizontally" with components (switches, buttons and pins) facing up. In this position, rotate the board, so that its convex shaped short side faces right. You'll find one USB connector (J2) on the left edge, another USB connector (J3) on the bottom edge and 4 buttons on the bottom right corner.

The board has several switches to configure its behavior. The out of the box configuration is the one we want. If the above instructions didn't work for you, check the position of the following switches:

- SW6 is set to the DEFAULT position (to the right - nRF = DEFAULT).
- SW7 (protected by Kapton tape) is set to the Def. position (to the right - TRACE = Def.).
- SW8 is set to the ON (to the left) position (Power = ON)
- SW9 is set to the VDD position (center - nRF power source = VDD)
- SW10 (protected by Kapton tape) is set to the OFF position (to the left - VEXT -> nRF = OFF).

![Labeled Diagram of the nRF52840 Development Kit (DK)](../img/nrf52840_dk_board.jpg)

## Detecting the board

We installed `cargo` when we set up the dongle, so we are able to use `cargo xtask usb-list`
to see whether the DK board is recognized. You should see something like this:

```console
❯ cargo xtask usb-list
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `xtask/target/debug/xtask usb-list`
Bus 003 Device 010: ID 1366:1061 <- J-Link on the nRF52840 Development Kit
```

## The nRF52840 chip

Both development boards have an nRF52840 microcontroller. Here are some details that are relevant to these exercises:

- Single core ARM Cortex-M4 processor clocked at 64 MHz
- 1 MB of Flash (at address `0x0000_0000`)
- 256 KB of RAM (at address `0x2000_0000`)
- IEEE 802.15.4 and BLE (Bluetooth Low Energy) compatible radio
- USB controller (device function)

## Preparing the flashing tool

To verify that our on-board J-Link is working properly and our board is ready for the
following exercises, we will flash a small hello world application onto it.

We are going to use a debugging tool built with Rust which is well integrated into the Embedded
Rust ecosystem called [`probe-rs`](https://probe.rs/). The [installation page](https://probe.rs/docs/getting-started/installation/)
specifies how you can install this tool on your operating system. If you are on Windows and have
problems executing the Windows PowerShell script, you can also download pre-built binaries from
the [releases page](https://github.com/probe-rs/probe-rs/releases). You can then place these pre-built
binaries at some location and add the location to your system PATH if it isn't there already.

You can use

```sh
probe-rs --version
```

to verify that you have `probe-rs` installed and available in your terminal.

Now, you might still have to do some operating system specific setup so that probe-rs can talk
to the on-board J-Link probe we saw earlier when we ran `cargo xtask usb-list`.

The [`probe-rs` setup page](https://probe.rs/docs/getting-started/probe-setup/) also specifies
these steps.

### Linux specific

You need the same `udev` rules that were setup [earlier](./dongle.md#linux-usb-access).

### Windows specific

No Windows specific steps should be necessary on more recent boards. If you have an older board
and `probe-rs` does not work properly, refer to [the troubleshooting for Windows chapter](./troubleshoot-windows-probe-rs.md).

## Flashing a test application

`probe-rs` is installed and you performed the OS specific steps so it can talk to our
MCU.

We provide a simple pre-built blinky app that can be used to quickly verify that this flashing
process works properly. You can download the `blinky` ELF file from the
[release](https://github.com/ferrous-systems/rust-exercises/releases) page.

Then, you can use the following command to flash the blinky binary to the board:

```sh
probe-rs run --chip nRF52840_xxAA --allow-erase-all ./blinky
```

You now should now see the LED1 blinking with a frequency of 1 second.

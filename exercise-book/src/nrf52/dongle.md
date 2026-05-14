# Dongle preparation

This chapter prepares the dongle board, which is the smaller of your two development boards based on the
nRF52840 chip.

The board has the form factor of a USB stick and can be directly connected to one of the USB ports
of your PC / laptop.

## Preparing some software

First of all, we want to prepare some software we need for displaying that our computer
detects the dongle. We have prepared a tool for this which is also cross-platform but using
this tool requires the [`cargo` package manager]. If you have not done so already, go to the
[Rust website](https://rust-lang.org/tools/install/) which should specify how to install Rust
for your operating system. Every Rust installation also comes with an installation of `cargo`.

After you have done this, you can use the following command in the terminal of your choice to check
your `cargo` installation:

```sh
cargo version
```

## Connecting the Dongle

Connect the Dongle to your PC/laptop. Its red LED should start oscillating in intensity.

Now, you can use the command `cargo xtask usb-list` to see something like this:

```sh
(...)
Bus 001 Device 011: ID 1915:521f <- nRF52840 Dongle (in bootloader mode)
(...)
```

## Updating the firmware

The Dongle does not contain an on-board debugger, like the larger DK board, so we cannot use
flashing tools like `probe-rs` to write programs into it. Instead, the Dongle's stock firmware
comes with a *bootloader*.

When put in bootloader mode the Dongle will run a bootloader program instead of the last application that was flashed into it. This bootloader program will make the Dongle show up as a USB CDC ACM device (AKA Serial over USB device) that accepts new application images over this interface. We'll use the `nrfdfu` tool to communicate with the bootloader-mode Dongle and flash new images into it.

`nrfdfu` is a Rust application which is published on the [`crates-io` Rust package registry](https://crates.io/).
This means that we can install it using `cargo`:

```console
cargo install nrfdfu
```

✅ Connect the Dongle to your computer. Put the Dongle in bootloader mode by  pressing its *reset* button.

 **💬 How to find the buttons on the Dongle:** Put the Dongle in front of you, so that the side with the parts mounted on faces up. Rotate it, so that the narrower part of the board, the surface USB connector, faces away from you.
The Dongle has two buttons. They are next to each other in the lower left corner of the Dongle. The reset button (RESET) is mounted sideways, it's square shaped button faces you. Further away from you is the round-ish user button (SW1), which faces up.

When the Dongle is in bootloader mode its red LED will pulsate. The Dongle will also appear as a USB CDC ACM device with vendor ID `0x1915` and product ID `0x521f`.

Now that the device is in bootloader mode, you need to get the Dongle Firmware.

❗️ This firmware will not be found in the git checkout - you need to get it from <https://github.com/ferrous-systems/rust-exercises/releases>.

You can download the individual firmware files from the [releases](https://github.com/ferrous-systems/rust-exercises/releases)
page. You need `dongle-fw`.

✅ Change to the directory where the `dongle-fw` file is located and run:

```console
nrfdfu ./dongle-fw
```

Expected output:

```console
[INFO  nrfdfu] Sending init packet...
[INFO  nrfdfu] Sending firmware image of size 37328...
[INFO  nrfdfu] Done.
```

After the device has been programmed it will automatically reset and start running the new application.

🔎 Alternatively, you can also use nordic's own [`nrfutil`](https://www.nordicsemi.com/Products/Development-tools/nRF-Util) tool to convert a .hex file and flash it for you, among many other things `nrfutil` is a very powerful tool, but also unstable at times, which is why we replaced the parts we needed from it with `nrfdfu`.

🔎 The `dongle-fw` application will make the Dongle enumerate itself as a CDC ACM device.

✅ Run `cargo xtask usb-list` to see the newly enumerated Dongle in the output:

```console
❯ cargo xtask usb-list
(...)
Bus 001 Device 009: ID 1209:0003 <- nRF52840 Dongle (dongle-fw)
(...)
```

The `dongle` app will log messages over the USB interface. To display these messages on the host we have provided a cross-platform tool: `cargo xtask serial-term`.

❗ Do not use serial terminal emulators like `minicom` or `screen`. They use the USB TTY ACM interface in a slightly different manner and may result in data loss.

✅ Run `cargo xtask serial-term` from the root of the extracted tarball / git checkout. It shows you the logging output the Dongle is sending on its serial interface to your computer. This helps you monitor what's going on at the Dongle and debug connection issues. Start with the Dongle unplugged and you should see the following output:

```console
$ cargo xtask serial-term
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `xtask/target/debug/xtask serial-term`
(waiting for the Dongle to be connected)
(..)
rx=0, err=0, ch=20, app=dongle-fw
(..)
```

This line is printed by the `dongle` app on boot. It contains the device ID of the dongle, a 64-bit unique identifier (so everyone will see a different number); the radio channel that the device will use to communicate; and the transmission power of the radio in dBm.

If you don't get any output from `cargo xtask serial-term` check [the USB dongle troubleshooting section][usb-issues].

[usb-issues]: troubleshoot-usb-dongle.md

The `dongle-fw` has 2 different modes, a puzzle mode and a loopback mode. The LED will glow green in
the loopback mode, and blue in the puzzle mode. You can use the larger user button to switch between
modes. We will need the loopback mode first, so make sure that the LED is glowing green, and press
the button to switch to the correct mode if necessary.

## Interference

At this point you should *not* get more output from `cargo xtask serial-term`.

❗If you get "received N bytes" lines in output like this:

```console
$ cargo xtask serial-term
rx=0, err=0, ch=20, app=dongle-fw
(..)
received 7 bytes (CRC=Ok(0x2459), LQI=0)
received 5 bytes (CRC=Ok(0xdad9), LQI=0)
received 6 bytes (CRC=Ok(0x72bb), LQI=0)
```

That means the device is observing interference traffic, likely from 2.4 GHz Zigbee, WiFi or Bluetooth. In this scenario you should switch the listening channel to one where you don't observe interference. Use the `cargo xtask change-channel` tool to do this in a second window. The tool takes a single argument: the new listening channel which must be in the range 11-26.

```console
$ cargo xtask change-channel 11
requested channel change to channel 11
```

Then you should see new output from `cargo xtask serial-term`:

```console
rx=0, err=0, ch=20, app=dongle-fw
(..)
now listening on channel 11
```

Leave the Dongle connected and `cargo xtask serial-term` running. Now we'll switch back to the
Development Kit. Note that if you remove and re-insert the dongle, it goes back to its default channel of 20.
You will also need to restart `cargo xtask serial-term`.

## Continuing with the board

We will continue with setting up the larger DK board now.

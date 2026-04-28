# nRF52840 Dongle

This is the smaller development board.

The board has the form factor of a USB stick and can be directly connected to one of the USB ports
of your PC / laptop.

## Connecting the Dongle

Connect the Dongle to your PC/laptop. Its red LED should start oscillating in intensity.

### Windows

The device shows up as a USB Serial Device (COM port) in the Device Manager under the Ports section

### Linux

The dongle shows up as a USB device under `lsusb`. The device will have a VID of `0x1915` and a PID of `0x521f` -- the `0x` prefix will be omitted in the output of `lsusb`:

```console
$ cyme
(..)
  1   8  0x1915 0x521f Open DFU Bootloader         F6D330A50560 -       12.0 Mb/s
```

```console
$ lsusb
(..)
Bus 001 Device 023: ID 1915:521f Nordic Semiconductor ASA 4-Port USB 2.0 Hub
```

The device will also show up in the `/dev` directory as a `ttyACM` device:

```console
$ ls -l /dev/serial/by-id/*
(..)
lrwxrwxrwx - root  2 Mar 14:37 /dev/serial/by-id/usb-Nordic_Semiconductor_Open_DFU_Bootloader_F6D330A50560-if00 -> ../../ttyACM2
```

### macOS

The device shows up as a usb device when executing `ioreg -p IOUSB -b -n "Open DFU Bootloader"`. The device will have a vendor ID (`"idVendor"`) of `6421` and a product ID (`"idProduct"`) of `21023`:

```console
$ ioreg -p IOUSB -b -n "Open DFU Bootloader"
(...)
| +-o Open DFU Bootloader@14300000  <class AppleUSBDevice, id 0x100005d5b, registered, matched, ac$
  |     {
  |       (...)
  |       "idProduct" = 21023
  |       (...)
  |       "USB Product Name" = "Open DFU Bootloader"
  |       (...)
  |       "USB Vendor Name" = "Nordic Semiconductor"
  |       "idVendor" = 6421
  |       (...)
  |       USB Serial Number" = "CA1781C8A1EE"
  |       (...)
  |     }
  |
```

The device will also show up in the `/dev` directory as `tty.usbmodem<USB Serial Number>`:

```console
$ ls /dev/tty.usbmodem*
/dev/tty.usbmodemCA1781C8A1EE1
```

## Updating the firmware

The Dongle does not contain an on-board debugger, like the larger DK board, so we cannot use
flashing tools like `probe-rs` to write programs into it. Instead, the Dongle's stock firmware
comes with a *bootloader*.

When put in bootloader mode the Dongle will run a bootloader program instead of the last application that was flashed into it. This bootloader program will make the Dongle show up as a USB CDC ACM device (AKA Serial over USB device) that accepts new application images over this interface. We'll use the `nrfdfu` tool to communicate with the bootloader-mode Dongle and flash new images into it.

✅ Connect the Dongle to your computer. Put the Dongle in bootloader mode by  pressing its *reset* button.

 **💬 How to find the buttons on the Dongle:** Put the Dongle in front of you, so that the side with the parts mounted on faces up. Rotate it, so that the narrower part of the board, the surface USB connector, faces away from you.
The Dongle has two buttons. They are next to each other in the lower left corner of the Dongle. The reset button (RESET) is mounted sideways, it's square shaped button faces you. Further away from you is the round-ish user button (SW1), which faces up.

When the Dongle is in bootloader mode its red LED will pulsate. The Dongle will also appear as a USB CDC ACM device with vendor ID `0x1915` and product ID `0x521f`.

You can also use `cyme`, a cross-platform version of the `lsusb` tool, to check out the status of the Dongle.

✅ Run `cyme` to list all USB devices.

Output should look like this:

```console
$ cyme
(..)
  2  16  0x1915 0x521f Open DFU Bootloader      E1550CA275E7      12.0 Mb/s
(..)
```

The first two values depend on your host computer and which USB port you used, so they will be different for you. The hex-string is the device's unique ID and that will also be different.

Now that the device is in bootloader mode, you need to get the Dongle Firmware.

❗️ This firmware will not be found in the git checkout - you need to get it from <https://github.com/ferrous-systems/rust-exercises/releases>.

* If you have downloaded and unpacked the complete rust-exercises release zip file, the firmware will be in the [`nrf52-code/boards/dongle-fw`](../../../nrf52-code/boards/dongle-fw) directory.
* If not, you can download the individual firmware files from the [releases](https://github.com/ferrous-systems/rust-exercises/releases) page. You need `dongle-fw`.

For the next section you'll need to flash the `dongle-fw` file onto the Dongle.

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

✅ Run `cyme` to see the newly enumerated Dongle in the output:

```console
$ cyme
(..)
  2  16  0x1209 0x0002 Dongle Loopback          -                 12.0 Mb/s
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

The `dongle-fw` has 2 different modes, a puzzle mode and a loopback mode. the LED will glow green in
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

## Continuing with the board

We will continue with setting up the larger DK board now.

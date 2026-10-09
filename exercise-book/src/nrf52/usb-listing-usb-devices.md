# Listing USB Devices

You can use the `cargo xtask usb-list` command at the root of the `rust-exercises` to list all USB
devices. This tool will mark the nRF USB device specifically if the enumeration is working properly.

## Alternative: Cyme utility

The Rust ecosystem has created a lot of high quality tools as replacements for more
classic Unix tools. `cyme` is a generic application to list USB devices on our system. It is the
Rust version of `lsusb`.

If you have not installed `cyme` yet, you can use

```sh
cargo install cyme
```

to install the tool.

✅ To list all USB devices, run `cyme` from the top-level checkout.

```console
$ cyme
(...) random other USB devices will be listed
  3   7  0x1366 0x1061 J-Link                               001050238928 -       12.0 Mb/s
```

## Goal

The goal of this exercise is to get the nRF52840 SoC to show in this list. The embedded application will use the USB Vendor ID (VID) 0x1209 and USB Product ID (PID) 0x0001, as defined in [`nrf52-code/consts`](../../../nrf52-code/consts):

`cargo xtask usb-list`

```console
(...) other USB devices
Bus 003 Device 008: ID 1209:0001 <- nRF52840 on the nRF52840 Development Kit
```

Cyme:

```console
$ cyme
(...) random other USB devices will be listed
  3   7  0x1366 0x1061 J-Link                               001050238928 -       12.0 Mb/s
  3   8  0x1209 0x0001 pid.codes Test PID                   -            -       12.0 Mb/s
````

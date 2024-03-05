# Listing USB Devices

✅ To list all USB devices, run `cargo xtask usb-list` from the top-level checkout.

```console
$ cargo xtask usb-list
(...) random other USB devices will be listed
Bus 001 Device 010: ID 1366:1015 <- J-Link on the nRF52840 Development Kit
```

The goal of this workshop is to get the nRF52840 SoC to show in this list. The embedded application will use the USB Vendor ID (VID) and USB Product ID (PID) defined in [`nrf52-code/consts`](../../nrf52-code/consts); `cargo xtask usb-list` will highlight the USB device that matches that VID/PID pair, like this:

```console
$ cargo xtask usb-list
(...) random other USB devices will be listed
Bus 001 Device 010: ID 1366:1015 <- J-Link on the nRF52840 Development Kit
Bus 001 Device 059: ID 1209:0717 <- nRF52840 on the nRF52840 Development Kit
```

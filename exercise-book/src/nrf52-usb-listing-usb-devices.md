# Listing USB Devices

As we showed in [Preparation/Software Tools](./nrf52-tools.md), we can use `cyme` to list USB devices on our system.

✅ To list all USB devices, run `cyme` from the top-level checkout.

```console
$ cyme
(...) random other USB devices will be listed
  2  15  0x1366 0x1051 J-Link                   001050255503      12.0 Mb/s
```

The goal of this workshop is to get the nRF52840 SoC to show in this list. The embedded application will use the USB Vendor ID (VID) 0x1209 and USB Product ID (PID) 0x0001, as defined in [`nrf52-code/consts`](../../nrf52-code/consts):

```console
$ cyme
(...) random other USB devices will be listed
  2  15  0x1366 0x1051 J-Link                   001050255503      12.0 Mb/s
  2  16  0x1209 0x0001 composite_device         -                 12.0 Mb/s
````

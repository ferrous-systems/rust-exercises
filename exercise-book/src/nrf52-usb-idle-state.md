# Idle State

Once you have handled all the previously covered requests the device should be enumerated and remain idle awaiting for a new host request. Your logs may look like this:

```console
[DEBUG] USB: UsbReset @ 00:00:00.347259 (usb_4 src/bin/usb-4.rs:56)
[WARN ] USB reset condition detected (usb_4 src/bin/usb-4.rs:60)
[DEBUG] USB: UsbEp0Setup @ 00:00:00.389770 (usb_4 src/bin/usb-4.rs:56)
[DEBUG] SETUP: bmrequesttype: 0b00000000, brequest: 5, wlength: 0, windex: 0x0000, wvalue: 0x000a (usb_4 src/bin/usb-4.rs:88)
[INFO ] EP0: SetAddress { address: Some(10) } (usb_4 src/bin/usb-4.rs:99)
[DEBUG] USB: UsbEp0Setup @ 00:00:00.393066 (usb_4 src/bin/usb-4.rs:56)
[DEBUG] SETUP: bmrequesttype: 0b10000000, brequest: 6, wlength: 8, windex: 0x0000, wvalue: 0x0100 (usb_4 src/bin/usb-4.rs:88)
[INFO ] EP0: GetDescriptor { descriptor: Device, length: 8 } (usb_4 src/bin/usb-4.rs:99)
[DEBUG] EP0IN: start 8B transfer (dk dk/src/usbd.rs:59)
[DEBUG] USB: UsbEp0DataDone @ 00:00:00.393585 (usb_4 src/bin/usb-4.rs:56)
[INFO ] EP0IN: transfer complete (usb_4 src/bin/usb-4.rs:65)
[INFO ] EP0IN: transfer done (dk dk/src/usbd.rs:83)
[DEBUG] USB: UsbEp0Setup @ 00:00:00.394409 (usb_4 src/bin/usb-4.rs:56)
[DEBUG] SETUP: bmrequesttype: 0b10000000, brequest: 6, wlength: 18, windex: 0x0000, wvalue: 0x0100 (usb_4 src/bin/usb-4.rs:88)
[INFO ] EP0: GetDescriptor { descriptor: Device, length: 18 } (usb_4 src/bin/usb-4.rs:99)
[DEBUG] EP0IN: start 18B transfer (dk dk/src/usbd.rs:59)
[DEBUG] USB: UsbEp0DataDone @ 00:00:00.394958 (usb_4 src/bin/usb-4.rs:56)
[INFO ] EP0IN: transfer complete (usb_4 src/bin/usb-4.rs:65)
[INFO ] EP0IN: transfer done (dk dk/src/usbd.rs:83)
[DEBUG] USB: UsbEp0Setup @ 00:00:00.395385 (usb_4 src/bin/usb-4.rs:56)
[DEBUG] SETUP: bmrequesttype: 0b10000000, brequest: 6, wlength: 9, windex: 0x0000, wvalue: 0x0200 (usb_4 src/bin/usb-4.rs:88)
[INFO ] EP0: GetDescriptor { descriptor: Configuration { index: 0 }, length: 9 } (usb_4 src/bin/usb-4.rs:99)
[DEBUG] EP0IN: start 9B transfer (dk dk/src/usbd.rs:59)
[DEBUG] USB: UsbEp0DataDone @ 00:00:00.396057 (usb_4 src/bin/usb-4.rs:56)
[INFO ] EP0IN: transfer complete (usb_4 src/bin/usb-4.rs:65)
[INFO ] EP0IN: transfer done (dk dk/src/usbd.rs:83)
[DEBUG] USB: UsbEp0Setup @ 00:00:00.396270 (usb_4 src/bin/usb-4.rs:56)
[DEBUG] SETUP: bmrequesttype: 0b10000000, brequest: 6, wlength: 18, windex: 0x0000, wvalue: 0x0200 (usb_4 src/bin/usb-4.rs:88)
[INFO ] EP0: GetDescriptor { descriptor: Configuration { index: 0 }, length: 18 } (usb_4 src/bin/usb-4.rs:99)
[DEBUG] EP0IN: start 18B transfer (dk dk/src/usbd.rs:59)
[DEBUG] USB: UsbEp0DataDone @ 00:00:00.396942 (usb_4 src/bin/usb-4.rs:56)
[INFO ] EP0IN: transfer complete (usb_4 src/bin/usb-4.rs:65)
[INFO ] EP0IN: transfer done (dk dk/src/usbd.rs:83)
[DEBUG] USB: UsbEp0Setup @ 00:00:00.401824 (usb_4 src/bin/usb-4.rs:56)
[DEBUG] SETUP: bmrequesttype: 0b00000000, brequest: 9, wlength: 0, windex: 0x0000, wvalue: 0x002a (usb_4 src/bin/usb-4.rs:88)
[INFO ] EP0: SetConfiguration { value: Some(42) } (usb_4 src/bin/usb-4.rs:99)
[WARN ] EP0IN: unexpected request; stalling the endpoint (usb_4 src/bin/usb-4.rs:71)
```

Note that these logs are from a macOS host where a `SET_ADDRESS` request is sent first, and then a `GET_DESCRIPTOR` request. On other OSes the messages may be in a different order. Also note that there are some `GET_DESCRIPTOR DeviceQualifier` requests in this case; you do not need to parse them in the `usb` crate as they'll be rejected (stalled) anyways.

You can find traces for other OSes in these files (they are in the [`nrf52-code/usb-app-solutions/traces`](../../nrf52-code/usb-app-solutions/traces) folder):

- `linux-enumeration.txt`
- `macos-enumeration.txt` (same logs as the ones shown above)
- `win-enumeration.txt`

✅ Double check that the enumeration works by running `cargo xtask usb-list`](./nrf52-tools.md) while `usb-4.rs` is running.

```console
$ cargo xtask usb-list
(...) random other USB devices will be listed
Bus 004 Device 001: ID 1209:0717 <-- nRF52840 on the nRF52840 Development Kit
```

You can also try `cyme`, but we've found that on Windows, the device may not appear in the tool's output. Possibly this is because it's only showing devices which have accepted a Configuration.

You can find a working solution up to this point in [`nrf52-code/usb-app-solutions/src/bin/usb-4.rs`](../../nrf52-code/usb-app-solutions/src/bin/usb-4.rs). Note that the solution uses the `usb2` crate to parse SETUP packets and that crate supports parsing all standard requests.

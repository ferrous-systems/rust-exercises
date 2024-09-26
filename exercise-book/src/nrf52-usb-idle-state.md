# Idle State

Once you have handled all the previously covered requests the device should be enumerated and remain idle awaiting for a new host request. Your logs may look like this:

```console
INFO:usb_4 -- USB: UsbReset @ 318.66455ms
INFO:usb_4 -- USB reset condition detected
INFO:usb_4 -- USB: UsbEp0Setup @ 391.418456ms
INFO:usb_4 -- EP0: GetDescriptor { descriptor: Device, length: 64 }
INFO:dk::usbd -- EP0IN: start 18B transfer
INFO:usb_4 -- USB: UsbEp0DataDone @ 391.723632ms
INFO:usb_4 -- EP0IN: transfer complete
INFO:dk::usbd -- EP0IN: transfer done
INFO:usb_4 -- USB: UsbReset @ 442.016601ms
INFO:usb_4 -- USB reset condition detected
INFO:usb_4 -- USB: UsbEp0Setup @ 514.709471ms
INFO:usb_4 -- EP0: SetAddress { address: Some(17) }
INFO:usb_4 -- USB: UsbEp0Setup @ 531.37207ms
INFO:usb_4 -- EP0: GetDescriptor { descriptor: Device, length: 18 }
INFO:dk::usbd -- EP0IN: start 18B transfer
INFO:usb_4 -- USB: UsbEp0DataDone @ 531.646727ms
INFO:usb_4 -- EP0IN: transfer complete
INFO:dk::usbd -- EP0IN: transfer done
INFO:usb_4 -- USB: UsbEp0Setup @ 531.829832ms
INFO:usb_4 -- EP0: GetDescriptor { descriptor: DeviceQualifier, length: 10 }
ERROR:usb_4 -- EP0IN: unexpected request; stalling the endpoint
INFO:usb_4 -- USB: UsbEp0Setup @ 532.226562ms
INFO:usb_4 -- EP0: GetDescriptor { descriptor: DeviceQualifier, length: 10 }
ERROR:usb_4 -- EP0IN: unexpected request; stalling the endpoint
INFO:usb_4 -- USB: UsbEp0Setup @ 532.592772ms
INFO:usb_4 -- EP0: GetDescriptor { descriptor: DeviceQualifier, length: 10 }
ERROR:usb_4 -- EP0IN: unexpected request; stalling the endpoint
INFO:usb_4 -- USB: UsbEp0Setup @ 533.020018ms
INFO:usb_4 -- EP0: GetDescriptor { descriptor: Configuration { index: 0 }, length: 9 }
INFO:dk::usbd -- EP0IN: start 9B transfer
INFO:usb_4 -- USB: UsbEp0DataDone @ 533.386228ms
INFO:usb_4 -- EP0IN: transfer complete
INFO:dk::usbd -- EP0IN: transfer done
INFO:usb_4 -- USB: UsbEp0Setup @ 533.569335ms
INFO:usb_4 -- EP0: GetDescriptor { descriptor: Configuration { index: 0 }, length: 18 }
INFO:dk::usbd -- EP0IN: start 18B transfer
INFO:usb_4 -- USB: UsbEp0DataDone @ 533.935546ms
INFO:usb_4 -- EP0IN: transfer complete
INFO:dk::usbd -- EP0IN: transfer done
INFO:usb_4 -- USB: UsbEp0Setup @ 534.118651ms
INFO:usb_4 -- EP0: SetConfiguration { value: Some(42) }
ERROR:usb_4 -- EP0IN: unexpected request; stalling the endpoint
```

Note that these logs are from a Linux host where a `SET_CONFIGURATION` request is sent after the `SET_ADDRESS` request. On other OSes you may not get that request before the bus goes idle. Also note that there are some `GET_DESCRIPTOR DeviceQualifier` requests in this case; you do not need to parse them in the `usb` crate as they'll be rejected (stalled) anyways.

You can find traces for other OSes in these files (they are in the [`nrf52-code/usb-app-solutions/traces`](../../nrf52-code/usb-app-solutions/traces) folder):

- `linux-enumeration.txt` (same logs as the ones shown above)
- `macos-enumeration.txt`
- `win-enumeration.txt`

✅ Double check that the enumeration works by running [`cyme`](./nrf52-usb-listing-usb-devices.md) while `usb-4.rs` is running.

```console
$ cyme
(...) random other USB devices will be listed
  2  15  0x1366 0x1051 J-Link                   001050255503      12.0 Mb/s
  2  16  0x1209 0x0717 composite_device         -                 12.0 Mb/s
```

Both the J-Link and the device implemented by your firmware should appear in the list.

You can find a working solution up to this point in [`nrf52-code/usb-app-solutions/src/bin/usb-4.rs`](../../nrf52-code/usb-app-solutions/src/bin/usb-4.rs). Note that the solution uses the `usb2` crate to parse SETUP packets and that crate supports parsing all standard requests.

# Getting it Configured

At this stage the device will be in the `Address` stage. It has been identified and enumerated by the host but cannot yet be used by host applications. The device must first move to the `Configured` state before the host can start, for example, HID communication or send non-standard requests over the control endpoint.

There is no template for this step - start with your solution to USB-4.

Windows will enumerate the device but not automatically configure it after enumeration. Here's what you should do to force the host to configure the device.

## Linux and macOS

Nothing extra needs to be done if you're working on a Linux or macOS host. The host will automatically send a `SET_CONFIGURATION` request so proceed to the `SET_CONFIGURATION` section to see how to handle the request.

## Windows

After getting the device enumerated and into the idle state, open the Zadig tool (covered in the setup instructions; see the top README) and use it to associate the nRF52840 USB device to the WinUSB driver. The nRF52840 will appear as a "unknown device" with a VID and PID that matches the ones defined in the [`consts` crate](../../nrf52-code/consts/src/lib.rs).

Now modify the `usb-descriptors` command within the `xtask` package to "open" the device -- this operation is commented out in the source code. With this modification `usb-descriptors` will cause Windows to send a `SET_CONFIGURATION` request to configure the device. You'll need to run `cargo xtask usb-descriptors` to test out the correct handling of the `SET_CONFIGURATION` request.

## SET_CONFIGURATION

The SET_CONFIGURATION request is sent by the host to configure the device. Its configuration according to section 9.4.7. of the [USB specification][usb_spec] is:

- `bmrequesttype` is **0b00000000**
- `brequest` is **9** (i.e. the SET_CONFIGURATION Request Code, see table 9-4 in the USB spec)
- `wValue` contains the requested configuration value
- `wIndex` and `wLength` are 0, there is no `wData`

[usb_spec]: ./nrf52-usb-usb-specification.md

✅ To handle a SET_CONFIGURATION, do the following:

- If the device is in the `Default` state, you should stall the endpoint because the operation is not permitted in that state.

- If the device is in the `Address` state, then
  - if `wValue` is 0 (`None` in the `usb` API) then stay in the `Address` state
  - if `wValue` is non-zero and valid (was previously reported in a configuration descriptor) then move to the `Configured` state
  - if `wValue` is not valid then stall the endpoint

- If the device is in the `Configured` state, then read the requested configuration value from `wValue`
  - if `wValue` is 0 (`None` in the `usb` API) then return to the `Address` state
  - if `wValue` is non-zero and valid (was previously reported in a configuration descriptor) then move to the `Configured` state with the new configuration value
  - if `wValue` is not valid then stall the endpoint

In all the cases where you did not stall the endpoint (by returning `Err`) you'll need to acknowledge the request by starting a STATUS stage.

✅ This is done by writing 1 to the TASKS_EP0STATUS register.

NOTE: On Windows, you may get a `GET_STATUS` request *before* the `SET_CONFIGURATION` request and although you *should* respond to it, stalling the `GET_STATUS` request seems sufficient to get the device to the `Configured` state.

## Expected output

✅ Run the progam and check the log output.

Once you are correctly handling the `SET_CONFIGURATION` request you should get logs like these:

```console
INFO:usb_5 -- USB: UsbReset @ 397.15576ms
INFO:usb_5 -- USB reset condition detected
INFO:usb_5 -- USB: UsbEp0Setup @ 470.00122ms
INFO:usb_5 -- EP0: GetDescriptor { descriptor: Device, length: 64 }
INFO:dk::usbd -- EP0IN: start 18B transfer
INFO:usb_5 -- USB: UsbEp0DataDone @ 470.306395ms
INFO:usb_5 -- EP0IN: transfer complete
INFO:dk::usbd -- EP0IN: transfer done
INFO:usb_5 -- USB: UsbReset @ 520.721433ms
INFO:usb_5 -- USB reset condition detected
INFO:usb_5 -- USB: UsbEp0Setup @ 593.292235ms
INFO:usb_5 -- EP0: SetAddress { address: Some(21) }
INFO:usb_5 -- USB: UsbEp0Setup @ 609.954832ms
INFO:usb_5 -- EP0: GetDescriptor { descriptor: Device, length: 18 }
INFO:dk::usbd -- EP0IN: start 18B transfer
INFO:usb_5 -- USB: UsbEp0DataDone @ 610.260008ms
INFO:usb_5 -- EP0IN: transfer complete
INFO:dk::usbd -- EP0IN: transfer done
INFO:usb_5 -- USB: UsbEp0Setup @ 610.443113ms
INFO:usb_5 -- EP0: GetDescriptor { descriptor: DeviceQualifier, length: 10 }
WARN:usb_5 -- EP0IN: stalled
INFO:usb_5 -- USB: UsbEp0Setup @ 610.809325ms
INFO:usb_5 -- EP0: GetDescriptor { descriptor: DeviceQualifier, length: 10 }
WARN:usb_5 -- EP0IN: stalled
INFO:usb_5 -- USB: UsbEp0Setup @ 611.175535ms
INFO:usb_5 -- EP0: GetDescriptor { descriptor: DeviceQualifier, length: 10 }
WARN:usb_5 -- EP0IN: stalled
INFO:usb_5 -- USB: UsbEp0Setup @ 611.511228ms
INFO:usb_5 -- EP0: GetDescriptor { descriptor: Configuration { index: 0 }, length: 9 }
INFO:dk::usbd -- EP0IN: start 9B transfer
INFO:usb_5 -- USB: UsbEp0DataDone @ 611.846922ms
INFO:usb_5 -- EP0IN: transfer complete
INFO:dk::usbd -- EP0IN: transfer done
INFO:usb_5 -- USB: UsbEp0Setup @ 612.030027ms
INFO:usb_5 -- EP0: GetDescriptor { descriptor: Configuration { index: 0 }, length: 18 }
INFO:dk::usbd -- EP0IN: start 18B transfer
INFO:usb_5 -- USB: UsbEp0DataDone @ 612.365721ms
INFO:usb_5 -- EP0IN: transfer complete
INFO:dk::usbd -- EP0IN: transfer done
INFO:usb_5 -- USB: UsbEp0Setup @ 612.640378ms
INFO:usb_5 -- EP0: SetConfiguration { value: Some(42) }
INFO:usb_5 -- entering the configured state
```

These logs are from a Linux host. You can find traces for other OSes in these files (they are in the `nrf52-code/usb-app-solutions/traces` folder):

- `linux-configured.txt` (same logs as the ones shown above)
- `win-configured.txt`, this file only contains the logs produced by running `cargo xtask usb-descriptors`
- `macos-configured.txt`

You can find a solution to this part of the exercise in `nrf52-code/usb-app-solutions/src/bin/usb-5.rs`.

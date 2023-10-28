# USB-3: DATA Stage

The next step is to respond to the GET_DESCRIPTOR request with a device descriptor.

❗️ Keep the cable connected to the J3 port for the rest of the workshop

## ✅ Open the `nrf52-code/usb-app/src/bin/usb-3.rs` file

Part of this response is already implemented. We'll go through this.

We'll use the `dk::usb::Ep0In` abstraction. An instance of it is available in the `board` value (inside the `#[init]` function). The first step is to make this `Ep0In` instance available to the `on_event` function.

The `Ep0In` API has two methods: `start` and `end`. `start` is used to start a DATA stage; this method takes a *slice of bytes* (`[u8]`) as argument; this argument is the response data. The `end` method needs to be called after `start`, when the EP0DATADONE event is raised, to complete the control transfer. `Ep0In` will automatically issue the STATUS stage that must follow the DATA stage.

## ✅ Handle the EP0DATADONE event

Do this by calling the `end` method of the `EP0In` API.

## ✅ Implement the response to the GET_DESCRIPTOR request.

Extend `usb-3.rs` so that it uses `Ep0In` to respond to the `GET_DESCRIPTOR Device` request (and only to that request).

### Values of the device descriptor

- `bLength = 18`, the size of the descriptor (must always be this value)
- `bDescriptorType = 1`, device descriptor type (must always be this value)
- `bDeviceClass = bDeviceSubClass = bDeviceProtocol = 0`, these are unimportant for enumeration
- `bMaxPacketSize0 = 64`, this is the most performant option (minimizes exchanges between the device and the host) and it's assumed by the `Ep0In` abstraction
- `idVendor = consts::VID`, value expected by `cargo xtask usb-list` (\*)
- `idProduct = consts::PID`, value expected by `cargo xtask usb-list` (\*)
- `bcdDevice = 0x0100`, this means version 1.0 but any value should do
- `iManufacturer = iProduct = iSerialNumber = None`, string descriptors not supported
- `bNumConfigurations = 1`, must be at least `1` so this is the minimum value

>(\*) the `common` crate refers to the crate in the `advanced/common` folder. It is already part of the `firmware` crate dependencies.

### Use the `usb2::device::Descriptor` abstraction

Although you can create the device descriptor by hand as an array filled with magic values we *strongly* recommend you use the `usb2::device::Descriptor` abstraction. The crate is already in the dependency list of the project; you can open its API documentation with the following command: `cargo doc -p usb2 --open`.

### The length of the device descriptor

The `usb2::device::Descriptor` struct does not have `bLength` and `bDescriptorType` fields. Those fields have fixed values according to the USB spec so you cannot modify or set them. When `bytes()` is called on the `Descriptor` value the returned array, the binary representation of the descriptor, will contain those fields set to their correct value.

The device descriptor is 18 bytes long but the host may ask for fewer bytes (see `wlength` field in the SETUP data). In that case you must respond with the amount of bytes the host asked for. The opposite may also happen: `wlength` may be larger than the size of the device descriptor; in this case your answer must be 18 bytes long (do *not* pad the response with zeroes).

### Expected log output

Once you have successfully responded to the GET_DESCRIPTOR Device request you should get logs like these (if you are logging like `usb-3` does):

```text
USB: UsbReset @ Duration { secs: 0, nanos: 211334227 }
USB: UsbEp0Setup @ Duration { secs: 0, nanos: 252380370 }
SETUP: bmrequesttype: 0, brequest: 5, wlength: 0, windex: 0, wvalue: 52
USB: UsbEp0Setup @ Duration { secs: 0, nanos: 254577635 }
SETUP: bmrequesttype: 128, brequest: 6, wlength: 8, windex: 0, wvalue: 256
GET_DESCRIPTOR Device [length=8]
EP0IN: start 8B transfer
USB: UsbEp0DataDone @ Duration { secs: 0, nanos: 254852293 }
EP0IN: transfer done
USB: UsbEp0Setup @ Duration { secs: 0, nanos: 257568358 }
SETUP: bmrequesttype: 128, brequest: 6, wlength: 18, windex: 0, wvalue: 256
GET_DESCRIPTOR Device [length=18]
EP0IN: start 18B transfer
USB: UsbEp0DataDone @ Duration { secs: 0, nanos: 257843016 }
EP0IN: transfer done
USB: UsbEp0Setup @ Duration { secs: 0, nanos: 259674071 }
SETUP: bmrequesttype: 128, brequest: 6, wlength: 9, windex: 0, wvalue: 512
ERROR unknown request (goal achieved if GET_DESCRIPTOR Device was handled before)
`dk::exit()` called; exiting ...
```

A solution to this exercise can be found in `nrf52-code/usb-app-solutions/src/bin/usb-3.rs`.

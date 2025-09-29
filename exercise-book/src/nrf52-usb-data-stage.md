# USB-3: DATA Stage

The next step is to respond to the `GET_DESCRIPTOR` request for our device descriptor, with an actual device descriptor that describes our USB Device.

## Handle the request

✅ Open the [`nrf52-code/usb-app/src/bin/usb-3.rs`][usb_3] file

Part of this response is already implemented. We'll go through this.

We'll use the `dk::usb::Ep0In` abstraction. An instance of it is available in the `board` value (inside the `#[init]` function). The first step is to make this `Ep0In` instance available to the `on_event` function.

The `Ep0In` API has two methods: `start` and `end`. `start` is used to start a DATA stage; this method takes a *slice of bytes* (`[u8]`) as argument; this argument is the response data. The `end` method needs to be called after `start`, when the EP0DATADONE event is raised, to complete the control transfer. `Ep0In` will automatically issue the STATUS stage that must follow the DATA stage.

✅ Handle the `EP0DATADONE` event

Do this by calling the `end` method on the `EP0In` instance.

✅ Implement the response to the `GET_DESCRIPTOR` request for device descriptors.

Extend [`nrf52-code/usb-app/src/bin/usb-3.rs`][usb_3] so that it uses `Ep0In` to respond to the `GET_DESCRIPTOR` request (but only for device descriptors - no other kind of descriptor).

### Values of the device descriptor

The raw values you need to pack into the descriptor are as follows. Note, we won't be doing this by hand, so read on before you start typing!

- `bLength = 18`, the size of the descriptor (must always be this value)
- `bDescriptorType = 1`, device descriptor type (must always be this value)
- `bDeviceClass = bDeviceSubClass = bDeviceProtocol = 0`, these are unimportant for enumeration
- `bMaxPacketSize0 = 64`, this is the most performant option (minimizes exchanges between the device and the host) and it's assumed by the `Ep0In` abstraction
- `idVendor = consts::USB_VID_DEMO`, our example's USB Vendor ID (\*)
- `idProduct = consts::USB_PID_RTIC_DEMO`, our example's USB Product ID (\*)
- `bcdDevice = 0x0100`, this means version 1.0 but any value should do
- `iManufacturer = iProduct = iSerialNumber = None`, string descriptors not supported
- `bNumConfigurations = 1`, must be at least `1` so this is the minimum value

>(\*) the `consts` crate refers to the crate in the [`nrf52-code/consts`](../../nrf52-code/consts) folder. It is already part of the `usb-app` crate dependencies.

### Use the `usb2::device::Descriptor` abstraction

Although you can create the device descriptor by hand as an array filled with magic values we *strongly* recommend you use the `usb2::device::Descriptor` abstraction. The crate is already in the dependency list of the project; browse to the `usb2` crate in the `cargo doc` output [you opened earlier](./nrf52-usb-api-documentation.md).

### The length of the device descriptor

The `usb2::device::Descriptor` struct does not have `bLength` and `bDescriptorType` fields. Those fields have fixed values according to the USB spec so you cannot modify or set them. When `bytes()` is called on the `Descriptor` value the returned array, the binary representation of the descriptor, will contain those fields set to their correct value.

The device descriptor is 18 bytes long but the host may ask for fewer bytes (see `wlength` field in the SETUP data). In that case you must respond with the amount of bytes the host asked for. The opposite may also happen: `wlength` may be larger than the size of the device descriptor; in this case your answer must be 18 bytes long (do *not* pad the response with zeroes).

### Expected log output

Once you have successfully responded to the GET_DESCRIPTOR Device request you should get logs like these (if you are logging like our solution does):

```text
00:00:00.367980 [INFO ] GET_DESCRIPTOR Device [length=64] (usb_3 src/bin/usb-3.rs:86)
00:00:00.368835 [ERROR] panicked at src/bin/usb-3.rs:60:34:
not yet implemented (usb_app src/lib.rs:8)
`dk::fail()` called; exiting ...
```

A solution to this exercise can be found in [`nrf52-code/usb-app-solutions/src/bin/usb-3.rs`](../../nrf52-code/usb-app-solutions/src/bin/usb-3.rs).

[usb_3]: ../../nrf52-code/usb-app/src/bin/usb-3.rs

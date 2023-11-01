# USB-2: SETUP Stage

At the end of program `usb-1` we received a `EP0SETUP` event. This event signals the *end* of the `SETUP` stage of a control transfer.  The nRF52840 USBD peripheral will automatically receive the `SETUP` data and store it in the registers `BMREQUESTTYPE`, `BREQUEST`, `WVALUE{L,H}`, `WINDEX{L,H}` and `WLENGTH{L,H}`.

In `usb-2.rs`, you will find a short description of each register above the variable into which it should be read.

> For in-depth register documentation, refer to Sections 6.35.13.31 to 6.35.13.38 of the [nRF52840 Product Specification][nrf product spec].

[nrf product spec]: https://infocenter.nordicsemi.com/pdf/nRF52840_PS_v1.1.pdf

## Writing a parser for the data of this SETUP stage

❗️ Keep the cable connected to the J3 port for the rest of the workshop

✅ Parse `GET_DESCRIPTOR` requests for `DEVICE` descriptors.

Modify `Request::parse()` in `nrf52-code/usb-lib/src/lib.rs` to recognize a `GET_DESCRIPTOR` request of type `DEVICE` so that the `get_descriptor_device` test passes. Note that the parser already handles `SET_ADDRESS` requests.

## Getting Started

### Writing code that can be tested

When you need to write some `no_std` code that does not involve device-specific I/O you should consider writing it as a separate crate. This way, you can test it on your development machine (e.g. `x86_64`) using the standard `cargo test` functionality.

So that's what we'll do here. In `nrf52-code/usb-lib/src/lib.rs` you'll find starter code for writing a `no_std` SETUP data parser. The starter code contains some unit tests; you can run them with `cargo test` (from within the `usb-lib` folder) or you can use Rust Analyzer's "Test" button in VS code.

The definition of `Descriptor::Configuration` as well as the associated test has been "commented out" using an `#[cfg(TODO)]` attribute because it is not handled by the firmware yet. Delete the `#[cfg(TODO)]` so that the unit tests can access it. This pattern is used for enum members and test functions throughout this workshop, so keep it in mind should you see it again.

### Description of GET_DESCRIPTOR request

We can recognize a GET_DESCRIPTOR request by the following properties:

- `bmRequestType` is **0b10000000**
- `bRequest` is **6** (i.e. the GET_DESCRIPTOR Request Code, defined in table 9-4 in the USB spec)

### Description of DEVICE descriptor requests

In this task, we only want to parse DEVICE descriptor requests. They have the following properties:

- the descriptor type is **1** (i.e. DEVICE, defined in table 9-5 of the USB spec)
- the descriptor index is **0**
- the wIndex is **0** for our purposes
- ❗️you need to fetch the descriptor type from the high byte of `wValue`, and the descriptor index from the the low byte of `wValue`

Check section 9.4.3 of the [USB specification][usb_spec] for a very detailed description of the requests. All the constants we'll be using are also described in Tables 9-3, 9-4 and 9-5 of the same document.

You should return `Err(Error::xxx)` if the properties aren't met.

🔎 Remember that you can:

- define binary literals by prefixing them with `0b`
- use bit shifts (`>>`) and casts (`as u8`) to get the high/low bytes of `wValue`

You will also find this information in the `// TODO implement ...` comment in the `Request::parse()` function of `lib.rs` file.
 > NOTE: If you'd like to learn more, take a look at Section 9.4.3 Get Descriptor of the USB specification.

See `nrf52-code/usb-lib-solution-get-device/src/lib.rs` for a solution.

✅ Read incoming request information and pass it to the parser:

modify `usb-2.rs` to read `USBD` registers and parse the SETUP data when an EP0SETUP event is received.

**Getting Started:**

- for a mapping of register names to the `USBD` API, check the entry for `nrf52840_hal::target::usbd` in the documentation you've created using `cargo doc`
- let bmrequesttype = usbd.bmrequesttype.read().bits() as u8;
- remember that we've learned how to read registers in `events.rs`.
- you will need to put together the higher and lower bits of `wlength`, `windex` and `wvalue` to get the whole field

- > Note: If you're using a Mac, you need to catch `SetAddress` requests returned by the parser as these are sent before the first GetDescriptor request. We added an empty handler for you already so there's nothing further to do (we're just explaining why it's there).

**Expected Result:**

When you have successfully received a GET_DESCRIPTOR request for a Device descriptor you are done. You should see an output like this:

```console
USB: UsbReset @ Duration { secs: 0, nanos: 361145018 }
USB: UsbEp0Setup @ Duration { secs: 0, nanos: 402465820 }
SETUP: bmrequesttype: 0, brequest: 5, wlength: 0, windex: 0, wvalue: 10
USB: UsbEp0Setup @ Duration { secs: 0, nanos: 404754637 }
SETUP: bmrequesttype: 128, brequest: 6, wlength: 8, windex: 0, wvalue: 256
GET_DESCRIPTOR Device [length=8]
Goal reached; move to the next section
`dk::exit()` called; exiting ...
```

> Note: `wlength` / `length` can vary depending on the OS, USB port (USB 2.0 vs USB 3.0) or the presence of a USB hub so you may see a different value.

You can find a solution to this step in `nrf52-code/usb-app-solution/src/bin/usb-2.rs`.

[usb_spec]: https://www.usb.org/document-library/usb-20-specification

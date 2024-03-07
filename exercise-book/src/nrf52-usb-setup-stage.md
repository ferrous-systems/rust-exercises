# USB-2: SETUP Stage

At the end of program `usb-1` we received a `EP0SETUP` event. This event signals the *end* of the `SETUP` stage of [a control transfer](./nrf52-usb-control-transfers.md).  The nRF52840 USBD peripheral will automatically receive the `SETUP` data and store it in the registers `BMREQUESTTYPE`, `BREQUEST`, `WVALUE{L,H}`, `WINDEX{L,H}` and `WLENGTH{L,H}`.

In [`nrf52-code/usb-app/src/bin/usb-2.rs`][usb_2], you will find a short description of each register above the variable into which it should be read. But before we read those registers, we need to write some parsing code and get it unit tested.

> For in-depth register documentation, refer to Sections 6.35.13.31 to 6.35.13.38 of the [nRF52840 Product Specification][nrf product spec].

[nrf product spec]: https://infocenter.nordicsemi.com/pdf/nRF52840_PS_v1.1.pdf

## Writing a parser for the data of this SETUP stage

We could parse the SETUP data inside our application, but it makes more sense to put the code in a library where we can test it, and where we can share it with other applications.

We have provided just such a library in [`nrf52-code/usb-lib`](../../nrf52-code/usb-lib/src/lib.rs). But it's missing some important parts that you need to complete. The definition of `Descriptor::Configuration` as well as the associated test has been "commented out" using an `#[cfg(TODO)]` attribute because it is not handled by the firmware yet - leave those disabled for the time being.

✅ Run `cargo test` in the [`nrf52-code/usb-lib`](../../nrf52-code/usb-lib/src/lib.rs) directory.

When you need to write some `no_std` code that does not involve device-specific I/O you should consider writing it as a separate crate. This way, you can test it on your development machine (e.g. `x86_64`) using the standard `cargo test` functionality.

So that's what we'll do here. In [`nrf52-code/usb-lib/src/lib.rs`](../../nrf52-code/usb-lib/src/lib.rs) you'll find starter code for writing a `no_std` SETUP data parser. The starter code contains some unit tests; you can run them with `cargo test` (from within the `usb-lib` folder) or you can use Rust Analyzer's "Test" button in VS code.

You should see:

```text
running 2 tests
test tests::set_address ... ok
test tests::get_descriptor_device ... FAILED

failures:

---- tests::get_descriptor_device stdout ----
thread 'tests::get_descriptor_device' panicked at src/lib.rs:119:9:
assertion `left == right` failed
  left: Err(UnknownRequest)
 right: Ok(GetDescriptor { descriptor: Device, length: 18 })
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::get_descriptor_device

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

---

✅ Fix the tests by parsing `GET_DESCRIPTOR` requests for `DEVICE` descriptors.

Modify `Request::parse()` in [`nrf52-code/usb-lib/src/lib.rs`](../../nrf52-code/usb-lib/src/lib.rs) to recognize a `GET_DESCRIPTOR` request of type `DEVICE` so that the `get_descriptor_device` test passes. Note that the parser already handles `SET_ADDRESS` requests.

### Description of GET_DESCRIPTOR request

We can recognize a GET_DESCRIPTOR request by the following properties:

- `bmRequestType` is **0b10000000**
- `bRequest` is **6** (i.e. the GET_DESCRIPTOR Request Code, defined in table 9-4 in the USB spec)

### Description of GET_DESCRIPTOR requests for DEVICE descriptors

In this step of the exercise, we only need to parse DEVICE descriptor requests. They have the following properties:

- the descriptor type is **1** (i.e. DEVICE, defined in table 9-5 of the USB spec)
- the descriptor index is **0**
- the wIndex is **0** for our purposes
- ❗️you need to fetch the descriptor type from the high byte of `wValue`, and the descriptor index from the the low byte of `wValue`

Check Section 9.4.3 of the [USB specification] for a very detailed description of the requests. All the constants we'll be using are also described in Tables 9-3, 9-4 and 9-5 of the same document. Or, you can refer to [Chapter 6 of USB In a Nutshell](https://www.beyondlogic.org/usbnutshell/usb6.shtml).

You should return `Err(Error::xxx)` if the properties aren't met.

🔎 Remember that you can:

- define binary literals by prefixing them with `0b`
- use bit shifts (`>>`) and casts (`as u8`) to get the high/low bytes of `wValue`

You will also find this information in the `// TODO implement ...` comment in the `Request::parse()` function of `lib.rs` file.

See [`nrf52-code/usb-lib-solutions/get-device/src/lib.rs`](../../nrf52-code/usb-lib-solutions/get-device/src/lib.rs) for a solution.

## Using our new parser

✅ Read incoming request information and pass it to the parser:

Modify [`nrf52-code/usb-app/src/bin/usb-2.rs`][usb_2] to read the appropriate `USBD` registers and parse them when an `EP0SETUP` event is received.

**Getting Started:**

- for a mapping of register names to the `USBD` API, check the entry for `nrf52840_hal::target::usbd` in the documentation you created using `cargo doc`
- Try `let value = usbd.register_name.read().bits() as u8;` if you just want the bottom eight bits of a register.
- remember that we've learned how to read registers in `events.rs`.
- you will need to put together the higher and lower bits of `wlength`, `windex` and `wvalue` to get the whole field, or use a library function to do it for you. Can the `dk` crate help?

- > Note: If you're using a Mac, you need to catch `SET_ADDRESS` requests returned by the parser as these are sent before the first `GET_DESCRIPTOR` request. We added an empty handler for you already so there's nothing further to do (we're just explaining why it's there).

**Expected Result:**

When you have successfully received a `GET_DESCRIPTOR` request for a Device descriptor you are done. You should see an output like this:

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

You can find a solution to this step in [`nrf52-code/usb-app-solutions/src/bin/usb-2.rs`](../../nrf52-code/usb-app-solutions/src/bin/usb-2.rs).

[USB specification]: ./nrf52-usb-usb-specification.md
[usb_2]: ../../nrf52-code/usb-app/src/bin/usb-2.rs

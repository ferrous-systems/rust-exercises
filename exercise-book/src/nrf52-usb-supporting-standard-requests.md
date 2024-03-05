# USB-4: Supporting more Standard Requests

After responding to the `GET_DESCRIPTOR Device` request the host will start sending different requests. Let's identify those, and then handle them.

## Update the parser

The starter `nrf52-code/usb-lib` package contains unit tests for everything we need. Some of them have been commented out using a `#[cfg(TODO)]` attribute.

✅ Remove all `#[cfg(TODO)]` attributes so that everything is enabled.

✅ Update the parser in `nrf52-code/usb-lib` to handle `GET_DESCRIPTOR` requests for *Configuration Descriptors*.

When the host issues a GET_DESCRIPTOR *Configuration* request the device needs to respond with the requested configuration descriptor *plus* all the interface and endpoint descriptors associated to that configuration descriptor during the DATA stage.

As a reminder, all GET_DESCRIPTOR request types share the following properties:

- `bmRequestType` is **0b10000000**
- `bRequest` is **6** (i.e. the GET_DESCRIPTOR Request Code, defined in table 9-4 of the [USB specification][usb_spec])

A GET_DESCRIPTOR *Configuration* request is determined by the high byte of its `wValue` field:

- The high byte of `wValue` is **2** (i.e. the `CONFIGURATION` descriptor type, defined in table 9-5 of the [USB specification][usb_spec])

[usb_spec]: ./nrf52-usb-usb-specification.md

✅ Update the parser in `nrf52-code/usb-lib` to handle `SET_CONFIGURATION` requests.

See the section on [SET_CONFIGURATION](./nrf52-usb-getting-device-configured.md#set_configuration) for details on how to do this.

Once you've completed this, all your test cases should pass. If not, fix the code until they do!

### Help

If you need a reference, you can find solutions to parsing `GET_DESCRIPTOR Configuration` and `SET_CONFIGURATION` requests in the following files:

- `nrf52-code/usb-lib-solutions/get-descriptor-config`
- `nrf52-code/usb-lib-solutions/set-config`

Each file contains just enough code to parse the request in its name and the `GET_DESCRIPTOR Device` and `SET_ADDRESS` requests. So you can refer to [`nrf52-code/usb-lib-solutions/get-descriptor-config`](../../nrf52-code/usb-lib-solutions/get-descriptor-config/src/lib.rs) without getting "spoiled" about how to parse the `SET_CONFIGURATION` request.

## Update the application

We're now going to be using [`nrf52-code/usb-app/src/bin/usb-4.rs`][usb_4].

[usb_4]: ../../nrf52-code/usb-app/src/bin/usb-4.rs

Since the logic of the `EP0SETUP` event handling is getting more complex with each added event, you can see that `usb-4.rs` was refactored to add error handling: the event handling now happens in a separate function *that returns a `Result`*. When it encounters an invalid host request, it returns the `Err` variant which can be handled by stalling the endpoint:

```rust ignore
fn on_event(/* parameters */) {
    match event {
        /* ... */
        Event::UsbEp0Setup => {
            if ep0setup(/* arguments */).is_err() {
                // unsupported or invalid request:
                // TODO add code to stall the endpoint
                defmt::warn!("EP0IN: unexpected request; stalling the endpoint");
            }
        }
    }
}

fn ep0setup(/* parameters */) -> Result<(), ()> {
    let req = Request::parse(/* arguments_*/)?;
    //                                       ^ early returns an `Err` if it occurs

    // TODO respond to the `req`; return `Err` if the request was invalid in this state

    Ok(())
}
```

Note that there's a difference between the error handling done here and the error handling commonly done in `std` programs. `std` programs usually bubble up errors to the top `main` function (using the `?` operator), report the error (or chain of errors) and then exit the application with a non-zero exit code. This approach is usually not appropriate for embedded programs as  

1. `main` cannot return,  
2. there may not be a console to print the error to and/or  
3. stopping the program, and e.g. requiring the user to reset it to make it work again, may not be desirable behavior.  

For these reasons in embedded software errors tend to be handled as early as possible rather than propagated all the way up.

This does not preclude error *reporting*. The above snippet includes error reporting in the form of a `defmt::warn!` statement. This log statement may not be included in the final release of the program as it may not be useful, or even visible, to an end user but it is useful during development.

✅ For each green test, extend [`usb-4.rs`][usb_4] to handle the new requests your parser is now able to recognize.

If that's all the information you need - go ahead! If you'd like some more detail, read on.

## Dealing with unknown requests: Stalling the endpoint

You may come across host requests other than the ones listed in previous sections.

For this situation, the USB specification defines a device-side procedure for "stalling an endpoint", which amounts to the device telling the host that it doesn't support some request.

> This procedure should be used to deal with invalid requests, requests whose `SETUP` stage doesn't match any USB 2.0 standard request, and requests not supported by the device – for instance the `SET_DESCRIPTOR` request is not mandatory.

✅ Use the `dk::usbd::ep0stall()` helper function to stall endpoint 0 in `nrf52-code/usb-app/src/bin/usb-4.rs` if an invalid request is received.

## Updating Device State

At some point during the initialization you'll receive a `SET_ADDRESS` request that will move the device from the `Default` state to the `Address` state. If you are working on Linux, you'll also receive a `SET_CONFIGURATION` request that will move the device from the `Address` state to the `Configured` state. Additionally, some requests are only valid in certain states– for example `SET_CONFIGURATION` is only valid if the device is in the `Address` state. For this reason `usb-4.rs` will need to keep track of the device's current state.

The device state should be tracked using a resource so that it's preserved across multiple executions of the `USBD` event handler. The `usb2` crate has a `State` enum with the 3 possible USB states: `Default`, `Address` and `Configured`. You can use that enum or roll your own.

✅ Start tracking and updating the device state to move your request handling forward.

### Update the handling of the `USBRESET` event

Instead of ignoring it, we now want it to change the state of the USB device. See section 9.1 USB Device States of the USB specification for details on what to do. Note that `fn on_event()` was given `state: &mut State`.

### Update the handling of `SET_ADDRESS` requests

> This request should come right after the `GET_DESCRIPTOR Device` request if you're using Linux, or be the first request sent to the device by macOS.

A SET_ADDRESS request has the following fields as defined by Section 9.4.6 Set Address of the USB spec:

- `bmrequesttype` is **0b00000000**
- `brequest` is **5** (i.e. the SET_ADDRESS Request Code, see table 9-4 in the USB spec)
- `wValue` contains the address to be used for all subsequent accesses
- `wIndex` and `wLength` are 0, there is no `wData`

It should be handled as follows:

- If the device is in the `Default` state, then
  - if the requested address stored in `wValue` was `0` (`None` in the `usb` API) then the device should stay in the `Default` state
  - otherwise the device should move to the `Address` state

- If the device is in the `Address` state, then
  - if the requested address stored in `wValue` was `0` (`None` in the `usb` API) then the device should return to the `Default` state
  - otherwise the device should remain in the `Address` state but start using the new address

- If the device is in the `Configured` state this request results in "unspecified" behavior according to the USB specification. You should stall the endpoint in this case.

> Note: According to the USB specification the device needs to respond to this request with a STATUS stage -- the DATA stage is omitted. The nRF52840 USBD peripheral will automatically issue the STATUS stage and switch to listening to the requested address (see the USBADDR register) so no interaction with the USBD peripheral is required for this request.
>
> For more details, read the introduction of section 6.35.9 of the nRF52840 Product Specification 1.0.

## Implement the handling of `GET_DESCRIPTOR Configuration` requests

So how should we respond to the host when it wants our Configuration Descriptor? As our only goal is to be enumerated we'll respond with the minimum amount of information possible.

✅ First, check the request

Configuration descriptors are requested by *index*, not by their configuration value. Since we reported a single configuration in our device descriptor the index in the request must be zero. Any other value should be rejected by stalling the endpoint (see section [Dealing with unknown requests: Stalling the endpoint](#dealing-with-unknown-requests-stalling-the-endpoint) for more information).

✅ Next, create and send a response

The response should consist of the configuration descriptor, followed by interface descriptors and then by (optional) endpoint descriptors. We'll include a minimal single interface descriptor in the response. Since endpoints are optional we will include none.

The configuration descriptor and one interface descriptor will be concatenated in a single packet so this response should be completed in a single DATA stage.

The configuration descriptor in the response should contain these fields:

- `bLength = 9`, the size of this descriptor (must always be this value)
- `bDescriptorType = 2`, configuration descriptor type (must always be this value)
- `wTotalLength = 18` = one configuration descriptor (9 bytes) and one interface descriptor (9 bytes)
- `bNumInterfaces = 1`, a single interface (the minimum value)
- `bConfigurationValue = 42`, any non-zero value will do
- `iConfiguration = 0`, string descriptors are not supported
- `bmAttributes { self_powered: true, remote_wakeup: false }`, self-powered due to the debugger connection
- `bMaxPower = 250` (500 mA), this is the maximum allowed value but any (non-zero?) value should do

The interface descriptor in the response should contain these fields:

- `bLength = 9`, the size of this descriptor (must always be this value)
- `bDescriptorType = 4`, interface descriptor type (must always be this value)
- `bInterfaceNumber = 0`, this is the first, and only, interface
- `bAlternateSetting = 0`, alternate settings are not supported
- `bNumEndpoints = 0`, no endpoint associated to this interface (other than the control endpoint)
- `bInterfaceClass = bInterfaceSubClass = bInterfaceProtocol = 0`, does not adhere to any specified USB interface
- `iInterface = 0`, string descriptors are not supported

Again, we strongly recommend that you use the `usb2::configuration::Descriptor` and `usb2::interface::Descriptor` abstractions here. Each descriptor instance can be transformed into its byte representation using the `bytes` method -- the method returns an array. To concatenate both arrays you can use an stack-allocated [`heapless::Vec`] buffer. If you haven't used the `heapless` crate before you can find example usage in the the [`src/bin/vec.rs`](../../nrf52-code/usb-app/src/bin/vec.rs) file.

> NOTE: the `usb2::configuration::Descriptor` and `usb2::interface::Descriptor` structs do not have `bLength` and `bDescriptorType` fields. Those fields have fixed values according to the USB spec so you cannot modify or set them. When `bytes()` is called on the `Descriptor` value, the returned array (which contains a binary representation of the descriptor, packed according to the USB 2.0 standard) will contain those fields set to their correct value.

[`heapless::Vec`]: https://docs.rs/heapless/0.8.0/heapless/struct.Vec.html

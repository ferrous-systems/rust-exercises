# Next Steps

## String descriptors

If you'd like to continue working on your workshop project, we recommend adding String Descriptors support to the USB firmware. To do this, follow these steps:

✅ Read through section 9.6.7 of the USB spec, which covers string descriptors.

✅ Change your *configuration* descriptor to use string descriptors. You'll want to change the `iConfiguration` field to a non-zero value. Note that this change will likely break enumeration.

✅ Re-run the program to see what new control requests you get from the host.

✅ Update the `usb` parser to handle the new requests.

✅ Extend the logic of `ep0setup` to handle these new requests.

 Eventually, you'll need to send a string descriptor to the host. Note here that Rust string literals are UTF-8 encoded but the USB protocol uses UTF-**16** strings. You'll need to convert between these formats.

✅ If this works, add strings to other descriptors like the device descriptor e.g. its `iProduct` field.

✅ To verify that string descriptors are working in a cross-platform way, extend the `cargo xtask usb-descriptors` program to also print the device's string descriptors. See the [`read_string_descriptor`] method but note that this must be called on a "device handle", which is what the commented out `open` operation does.

[`read_string_descriptor`]: https://docs.rs/rusb/0.6.2/rusb/struct.DeviceHandle.html#method.read_string_descriptor

## Explore more RTIC features

We have covered only a few of the core features of the RTIC framework but the framework has many more features like *software* tasks, tasks that can be spawned by the software; message passing between tasks; and task scheduling, which allows the creation of periodic tasks. We encourage to check the [RTIC book][rtic-book] which describes the features we haven't covered here.

[rtic-book]: https://rtic.rs/2/book/en/

## usb-device

[`usb-device`] is a library for building USB devices. It has been built using traits (the pillar of Rust's *generics*) such that USB interfaces like HID and TTY ACM can be implemented in a device agnostic manner. The device details then are limited to a trait *implementation*. There's an implementation of the `usb-device` trait for the nRF52840 device in [the `nrf-hal`] and there are many `usb-device` "classes" like [HID] and [TTY ACM] that can be used with that trait implementation. We encourage you to check out that implementation, test it on different OSes and report issues, or contribute fixes, to the `usb-device` ecosystem.

[the `nrf-hal`]: https://github.com/nrf-rs/nrf-hal
[HID]: https://crates.io/crates/usbd-hid
[TTY ACM]: https://crates.io/crates/usbd-serial

[`usb-device`]: https://crates.io/crates/usb-device

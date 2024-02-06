# USB-4: Supporting more Standard Requests

After responding to the `GET_DESCRIPTOR Device` request the host will start sending different requests.

✅ Update the parser in `nrf52-code/usb-lib` so that it can handle the following requests:

1. `GET_DESCRIPTOR Configuration`, see the section on [Handling GET_DESCRIPTOR Configuration Requests](./nrf52-usb-get-descriptor-config.md#handling-get_descriptor-configuration-requests)
2. `SET_CONFIGURATION`, see the section on [SET_CONFIGURATION](./nrf52-usb-getting-device-configured.md#set_configuration) of this course material

The starter `nrf52-code/usb-lib` package contains unit tests for these other requests as well as extra `Request` variants for these requests. All of them have been commented out using a `#[cfg(TODO)]` attribute which you can remove once you need any new variant or new unit test.

✅ For each green test, extend `usb-4.rs` to handle the new requests your parser is now able to recognize. **Make sure to read the next sections as you're working**, since they contain explanations about the concepts used and needed to complete this task.

❗️ Keep the cable connected to the J3 port for the rest of the workshop

If you need a reference, you can find solutions to parsing `GET_DESCRIPTOR Configuration` and `SET_CONFIGURATION` requests in the following files:

- `nrf52-code/usb-lib-solutions/get-descriptor-config`
- `nrf52-code/usb-lib-solutions/set-config`

Each file contains just enough code to parse the request in its name and the `GET_DESCRIPTOR Device` and `SET_ADDRESS` requests. So you can refer to `nrf52-code/usb-lib-solutions/get-descriptor-config` without getting "spoiled" about how to parse the `SET_CONFIGURATION` request.

# Handling SET_ADDRESS Requests

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

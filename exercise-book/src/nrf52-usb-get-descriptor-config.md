# Handling GET_DESCRIPTOR Configuration Requests

When the host issues a GET_DESCRIPTOR *Configuration* request the device needs to respond with the requested configuration descriptor *plus* all the interface and endpoint descriptors associated to that configuration descriptor during the DATA stage.

As a reminder, all GET_DESCRIPTOR request types are share the following properties:

- `bmRequestType` is **0b10000000**
- `bRequest` is **6** (i.e. the GET_DESCRIPTOR Request Code, defined in table 9-4 of the [USB specification][usb_spec])


A GET_DESCRIPTOR *Configuration* request is determined the high byte of its `wValue` field:

- The high byte of `wValue` is **2** (i.e. the `CONFIGURATION` descriptor type, defined in table 9-5 of the [USB specification][usb_spec])

[usb_spec]: https://www.usb.org/document-library/usb-20-specification

In the next sections, let's look into all the concepts required to respond to this request.

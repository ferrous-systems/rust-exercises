# Inspecting the Descriptors

There's a tool built into our `cargo xtask` called `usb-descriptors`, it prints all the descriptors reported by your application

## ✅ Run this tool

Your output should look like this:

```console
$ cargo xtask usb-descriptors
DeviceDescriptor {
    bLength: 18,
    bDescriptorType: 1,
    bcdUSB: 512,
    bDeviceClass: 0,
    bDeviceSubClass: 0,
    bDeviceProtocol: 0,
    bMaxPacketSize: 64,
    idVendor: 8224,
    idProduct: 1815,
    bcdDevice: 256,
    iManufacturer: 0,
    iProduct: 0,
    iSerialNumber: 0,
    bNumConfigurations: 1,
}
address: 22
config0: ConfigDescriptor {
    bLength: 9,
    bDescriptorType: 2,
    wTotalLength: 18,
    bNumInterfaces: 1,
    bConfigurationValue: 42,
    iConfiguration: 0,
    bmAttributes: 192,
    bMaxPower: 250,
    extra: None,
}
iface0: [
    InterfaceDescriptor {
        bLength: 9,
        bDescriptorType: 4,
        bInterfaceNumber: 0,
        bAlternateSetting: 0,
        bNumEndpoints: 0,
        bInterfaceClass: 0,
        bInterfaceSubClass: 0,
        bInterfaceProtocol: 0,
        iInterface: 0,
    },
]
```

The output above corresponds to the descriptor values we suggested. If you used different values, e.g. for `bMaxPower`, you'll a slightly different output.

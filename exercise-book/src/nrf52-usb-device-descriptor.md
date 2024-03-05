# USB Device Descriptors

![USB hierarchy diagram showing the relationship between configurations, interfaces and endpoints. The diagram consists of nested rectangles. In this version of the diagram the outermost 'device' rectangle and the 'bNumConfigurations' label are highlighted in blue. The outermost rectangle is labeled 'device' and represents the complete USB device. Inside the 'device' rectangle there is one rectangle labeled 'configuration 1'; this rectangle has a 'parallel lines' symbol that indicates there may be more than one configuration instance; the symbol is labeled 'bNumConfigurations=1' indicating that this device has only one configuration. Inside the 'configuration 1' rectangle there are two rectangles labeled 'control endpoint' and 'interface 0'. Inside the 'control endpoint' rectangle there are two rectangles labeled 'endpoint 0 IN' and 'endpoint 0 OUT. The 'interface 0' rectangle has a 'parallel lines' symbol that indicates there may be more than one interface instance; the symbol is labeled 'bNumInterfaces=1' indicating that this configuration has only one interface. Inside the 'interface 0' rectangle there are three rectangles labeled 'endpoint 1 IN', 'endpoint 2 IN' and 'endpoint 2 OUT'. Between these three rectangle there is a label that says 'bNumEndpoints=3'; it indicates that this interface has only three endpoints.](img/usb-device.svg)

After receiving a `GET_DESCRIPTOR` request during the __SETUP__ stage, the device needs to respond with the actual *descriptor* data during the __DATA__ stage. In our Rust application, this descriptor will be generated using some library code and serialised into an array of bytes which we can give to the USBD peripheral.

A descriptor is a binary encoded data structure sent by the device to the host. The device descriptor, in particular, contains information about the device, like its product and vendor identifiers and how many *configurations* it has. The format of the device descriptor is specified in section 9.6.1, Device, of the [USB specification][usb_spec].

[usb_spec]: ./nrf52-usb-usb-specification.md

As far as the enumeration process goes, the most relevant fields of the device descriptor are the number of configurations and `bcdUSB`, the version of the USB specification the devices adheres to. In `bcdUSB` you should report compatibility with USB 2.0.

What about (the number of) configurations?

A *configuration* is akin to an operation mode. USB devices usually have a single configuration that will be the only mode in which they'll operate, for example a USB mouse will always act as a USB mouse. Some devices, though, may provide a second configuration for the purpose of firmware upgrades. For example a printer may enter DFU (Device Firmware Upgrade) mode, a second *configuration*, so that a user can update its firmware; while in DFU mode the printer will not provide printing functionality.

The specification mandates that a device must have at least one available configuration so we can report a single configuration in the device descriptor.

You can read more about Device Descriptors in [Chapter 5 of USB In a Nutshell](https://www.beyondlogic.org/usbnutshell/usb5.shtml).

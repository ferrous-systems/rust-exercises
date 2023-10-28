# Interface descriptor

![USB hierarchy diagram showing the relationship between configurations, interfaces and endpoints. The diagram consists of nested rectangles. In this version of the diagram the 'interface 0' rectangle and the 'bNumEndpoints' label are highlighted in blue. The outermost rectangle is labeled 'device' and represents the complete USB device. Inside the 'device' rectangle there is one rectangle labeled 'configuration 1'; this rectangle has a 'parallel lines' symbol that indicates there may be more than one configuration instance; the symbol is labeled 'bNumConfigurations=1' indicating that this device has only one configuration. Inside the 'configuration 1' rectangle there are two rectangles labeled 'control endpoint' and 'interface 0'. Inside the 'control endpoint' rectangle there are two rectangles labeled 'endpoint 0 IN' and 'endpoint 0 OUT. The 'interface 0' rectangle has a 'parallel lines' symbol that indicates there may be more than one interface instance; the symbol is labeled 'bNumInterfaces=1' indicating that this configuration has only one interface. Inside the 'interface 0' rectangle there are three rectangles labeled 'endpoint 1 IN', 'endpoint 2 IN' and 'endpoint 2 OUT'. Between these three rectangle there is a label that says 'bNumEndpoints=3'; it indicates that this interface has only three endpoints.](./img/usb-interface.svg)

The interface descriptor describes one of the device interfaces to the host. The descriptor contains the following information about a particular interface:

- its interface number -- this is a zero-based index
- its alternate setting -- this allows configuring the interface
- its number of endpoints
- class, subclass and protocol -- these define the interface (HID, or TTY ACM, or DFU, etc.) according to the USB specification

The number of endpoints can be zero and endpoint zero must not be accounted when counting endpoints.

> The full format of the interface descriptor is specified in section 9.6.5, Interface, of the USB specification.

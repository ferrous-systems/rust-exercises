# USB Control Transfers

![USB hierarchy diagram showing the relationship between configurations, interfaces and endpoints. The diagram consists of nested rectangles. In this version of the diagram the 'control endpoint' rectangle is highlighted in blue. The outermost rectangle is labeled 'device' and represents the complete USB device. Inside the 'device' rectangle there is one rectangle labeled 'configuration 1'; this rectangle has a 'parallel lines' symbol that indicates there may be more than one configuration instance; the symbol is labeled 'bNumConfigurations=1' indicating that this device has only one configuration. Inside the 'configuration 1' rectangle there are two rectangles labeled 'control endpoint' and 'interface 0'. Inside the 'control endpoint' rectangle there are two rectangles labeled 'endpoint 0 IN' and 'endpoint 0 OUT. The 'interface 0' rectangle has a 'parallel lines' symbol that indicates there may be more than one interface instance; the symbol is labeled 'bNumInterfaces=1' indicating that this configuration has only one interface. Inside the 'interface 0' rectangle there are three rectangles labeled 'endpoint 1 IN', 'endpoint 2 IN' and 'endpoint 2 OUT'. Between these three rectangle there is a label that says 'bNumEndpoints=3'; it indicates that this interface has only three endpoints](img/usb-control.svg)

Before we continue we need to discuss how data transfers work under the USB protocol.

The *control pipe* handles *control transfers*, a special kind of data transfer used by the host to issue *requests*. A control transfer is a data transfer that occurs in three stages: a SETUP stage, an optional DATA stage and a STATUS stage. The device must handle these requests by either supplying the requested data, or performing the requested action.

During the SETUP stage the host sends 8 bytes of data that identify the control request. Depending on the issued request there may be a DATA stage or not; during the DATA stage data is transferred either from the device to the host or the other way around. During the STATUS stage the device acknowledges, or not, the whole control request.

For detailed information about control transfers see [Chapter 4 of USB In a Nutshell](https://www.beyondlogic.org/usbnutshell/usb4.shtml).

In this workshop, we expect the host to perform a control transfer to find out what kind of device we are.

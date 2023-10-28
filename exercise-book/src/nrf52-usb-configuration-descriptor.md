# Configuration descriptor

![USB hierarchy diagram showing the relationship between configurations, interfaces and endpoints. The diagram consists of nested rectangles. In this version of the diagram the 'configuration 1' rectangle and the 'bNumInterface' label are highlighted in blue. The outermost rectangle is labeled 'device' and represents the complete USB device. Inside the 'device' rectangle there is one rectangle labeled 'configuration 1'; this rectangle has a 'parallel lines' symbol that indicates there may be more than one configuration instance; the symbol is labeled 'bNumConfigurations=1' indicating that this device has only one configuration. Inside the 'configuration 1' rectangle there are two rectangles labeled 'control endpoint' and 'interface 0'. Inside the 'control endpoint' rectangle there are two rectangles labeled 'endpoint 0 IN' and 'endpoint 0 OUT. The 'interface 0' rectangle has a 'parallel lines' symbol that indicates there may be more than one interface instance; the symbol is labeled 'bNumInterfaces=1' indicating that this configuration has only one interface. Inside the 'interface 0' rectangle there are three rectangles labeled 'endpoint 1 IN', 'endpoint 2 IN' and 'endpoint 2 OUT'. Between these three rectangle there is a label that says 'bNumEndpoints=3'; it indicates that this interface has only three endpoints.](img/usb-configuration.svg)

The configuration descriptor describes one of the device configurations to the host. The descriptor contains the following information about a particular configuration:

- the total length of the configuration: this is the number of bytes required to transfer this configuration descriptor and the interface and endpoint descriptors associated to it
- its number of interfaces -- must be >= 1
- its configuration value -- this is *not* an index and can be any non-zero value
- whether the configuration is self-powered
- whether the configuration supports remote wakeup
- its maximum power consumption

> The full format of the configuration descriptor is specified in section 9.6.3, Configuration, of the USB specification.

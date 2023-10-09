# USB-1: Dealing with USB Events

The USBD peripheral on the nRF52840 contains a series of registers, called EVENTS registers, that indicate the reason for entering the USBD event handler. These events must be handled by the application to complete the enumeration process.

✅ Open the `nrf52-code/usb-app/src/bin/usb-1.rs` file.

In this starter code the USBD peripheral is initialized in `init` and a task, named `main`, is bound to the interrupt signal USBD. This task will be called every time a new USBD event needs to be handled. The `main` task uses `usbd::next_event()` to check all the event registers; if any event is set (occurred) then the function returns the event, represented by the `Event` enum, wrapped in the `Some` variant. This `Event` is then passed to the `on_event` function for further processing.

✅ Connect the USB cable to the port J3 then run the starter code.

❗️ Keep the cable connected to the J3 port for the rest of the workshop

This code will panic because `USBRESET` is not implemented yet.

✅ Go to `fn on_event(...)`, line 39. In this section you'll need to implement the following USB events `USBRESET` and `EP0SETUP` so that your log output will look like this:

``` console
USBD initialized
USB: UsbReset
returning to the `Default` state
USB: UsbEp0Setup
goal reached; move to the next section
```

## Help

- `USBRESET`. This event indicates that the host issued a USB reset signal. According to the USB specification this will move the device from any state to the `Default` state. Since we are currently not dealing with any other state, you can handle this state by adding a log statement to provide information that this event occurred.

- `EP0DATADONE`. The USBD peripheral is signaling the end of the DATA stage of a control transfer. Since you won't encounter this event just yet, you can leave it as it is.

- `EP0SETUP`. The USBD peripheral has detected the SETUP stage of a control transfer. Add a log statement containing "goal reached; move to the next section" and exit the application.

You can find the solution in the `usb-1-solution.rs` file.

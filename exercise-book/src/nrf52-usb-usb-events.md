# USB-1: Dealing with USB Events

The `USBD` peripheral on the nRF52840 contains a series of registers, called `EVENTS` registers, that indicate the reason for entering the USBD _interrupt handler_. These events must be handled by the application to complete the enumeration process.

✅ Open the [`nrf52-code/usb-app/src/bin/usb-1.rs`][usb_1] file.

In this starter code the `USBD` peripheral is initialized in `init` and a task, named `handle_usb_interrupt`, is bound to the interrupt signal called `USBD`. This task will be called every time a new `USBD` event needs to be handled. The `handle_usb_interrupt` task uses `usbd::next_event()` to check all the event registers; if any event is set (i.e. that event just occurred) then the function returns the event, represented by the `Event` enum, wrapped in the `Some` variant. This `Event` is then passed to the `on_event` function for further processing.

✅ Connect the USB cable to the port J3 then run the starter code.

❗️ Keep the cable connected to the J3 port for the rest of the workshop

This code will panic because `Event::UsbReset` is not handled yet - it has a `todo!()` on the relevant `match` arm.

✅ Go to `fn on_event(...)`, line 48. You'll need to handle the `Event::UsbReset` case - for now, just print the log message _returning to the Default state_.

✅ Now handle the `Event::UsbEp0Setup` case - for now, just print the log message _usb-1 exercise complete_ and then execute `dk::exit()` to shut down the microcontroller.

Your logs should look like:

```console
USBD initialized
USB: UsbReset
returning to the Default state
USB: UsbEp0Setup
usb-1 exercise complete
```

You can ignore the `Event::UsbEp0DataDone` event for now because we don't yet get far enough when talking to the host computer for this event to come up.

## USB Knowledge

### `USBRESET` (indicated by `Events::UsbReset`)

This event indicates that the host issued a USB reset signal - the first step in [the enumeration process](./nrf52-usb-usb-enumeration.md). According to the USB specification this will move the device from any state to the __Default__ state. Since we are currently not dealing with any other state, for now we just log that we received this event and move on.

### `EP0SETUP` (indicated by `Events::UsbEp0Setup`)

The `USBD` peripheral has detected the SETUP stage of a control transfer. For now, we just print a log message and exit the application.

### `EP0DATADONE` (indicated by `Events::UsbEp0DataDone`)

The `USBD` peripheral is signaling the end of the DATA stage of a control transfer. Since you won't encounter this event just yet, you can leave it as it is.

## Help

You can find the solution in the [`nrf52-code/usb-app-solutions/src/bin/usb-1.rs`][usb_1] file.

[usb_1]: ../../nrf52-code/usb-app-solutions/src/bin/usb-1.rs

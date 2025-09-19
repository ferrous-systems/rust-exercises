#![no_main]
#![no_std]

use dk::hal::pac::usbd::Usbd;
use dk::usbd::{self, Event};

// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use usb_app as _;

#[rtic::app(device = dk, peripherals = false)]
mod app {

    use super::*;

    #[local]
    struct MyLocalResources {
        usbd: Usbd,
    }

    #[shared]
    struct MySharedResources {}

    #[init]
    fn init(_cx: init::Context) -> (MySharedResources, MyLocalResources) {
        let board = dk::init().unwrap();

        // initialize the USBD peripheral
        // NOTE this will block if the USB cable is not connected to port J3
        usbd::init(board.power, &board.usbd);

        defmt::println!("USBD initialized");

        (MySharedResources {}, MyLocalResources { usbd: board.usbd })
    }

    #[task(binds = USBD, local = [usbd])]
    fn handle_usb_interrupt(cx: handle_usb_interrupt::Context) {
        while let Some(event) = usbd::next_event(cx.local.usbd) {
            on_event(cx.local.usbd, event)
        }
    }
}

/// Handle a USB event (in interrupt context)
fn on_event(_usbd: &Usbd, event: Event) {
    defmt::debug!("USB: {} @ {=u64:tus}", event, dk::uptime_us());

    match event {
        Event::UsbReset => {
            defmt::println!("returning to the Default state");
        }
        Event::UsbEp0Setup => {
            defmt::println!("usb-1 exercise complete");
            dk::exit();
        }
        Event::UsbEp0DataDone => todo!(),
    }
}

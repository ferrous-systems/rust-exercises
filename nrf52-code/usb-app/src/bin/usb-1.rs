#![no_main]
#![no_std]

// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use usb_app as _;

#[rtic::app(device = dk, peripherals = false)]
mod app {
    use dk::{
        peripheral::USBD,
        usbd::{self, Event},
    };

    #[local]
    struct MyLocalResources {
        usbd: USBD,
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
        let usbd = cx.local.usbd;

        while let Some(event) = usbd::next_event(usbd) {
            on_event(usbd, event)
        }
    }

    fn on_event(_usbd: &USBD, event: Event) {
        defmt::debug!("USB: {} @ {=u64:tus}", event, dk::uptime_us());

        match event {
            Event::UsbReset => todo!(),
            Event::UsbEp0Setup => todo!(),
            Event::UsbEp0DataDone => todo!(),
        }
    }
}

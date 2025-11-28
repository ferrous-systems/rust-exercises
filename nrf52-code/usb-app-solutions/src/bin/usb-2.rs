#![no_main]
#![no_std]

use dk::hal::pac::usbd::Usbd;
use dk::usbd::{self, Event};

use usb::{Descriptor, Request};

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

        usbd::init(board.power, &board.usbd);

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
fn on_event(usbd: &Usbd, event: Event) {
    defmt::debug!("USB: {} @ {=u64:tus}", event, dk::uptime_us());

    match event {
        Event::UsbReset => {
            // nothing to do here at the moment
        }

        Event::UsbEp0DataDone => todo!(),

        Event::UsbEp0Setup => {
            // the BMREQUESTTYPE register contains information about data recipient, transfer type and direction
            let bmrequesttype = usbd.bmrequesttype().read().0 as u8;
            // the BREQUEST register stores the type of the current request (e.g. SET_ADDRESS, GET_DESCRIPTOR, ...)
            let brequest = usbd.brequest().read().brequest() as u8;
            // wLength denotes the number of bytes to transfer (if any)
            // composed of a high register (WLENGTHH) and a low register (WLENGTHL)
            let wlength = (u16::from(usbd.wlengthh().read().wlengthh()) << 8)
                | u16::from(usbd.wlengthl().read().wlengthl());
            // wIndex is a generic index field whose meaning depends on the request type
            // composed of a high register (WINDEXH) and a low register (WINDEXL)
            let windex = (u16::from(usbd.windexh().read().windexh()) << 8)
                | u16::from(usbd.windexl().read().windexl());
            // wValue is a generic parameter field meaning depends on the request type (e.g. contains the device
            // address in SET_ADDRESS requests)
            // composed of a high register (WVALUEH) and a low register (WVALUEL)
            let wvalue = (u16::from(usbd.wvalueh().read().wvalueh()) << 8)
                | u16::from(usbd.wvaluel().read().wvaluel());

            // NOTE the `dk` crate contains helper functions for the above operations
            // let bmrequesttype = usbd::bmrequesttype(usbd);
            // let brequest = usbd::brequest(usbd);
            // let wlength = usbd::wlength(usbd);
            // let windex = usbd::windex(usbd);
            // let wvalue = usbd::wvalue(usbd);

            defmt::info!(
                    "SETUP: bmrequesttype: 0b{=u8:08b}, brequest: {=u8}, wlength: {=u16}, windex: 0x{=u16:04x}, wvalue: 0x{=u16:04x}",
                    bmrequesttype,
                    brequest,
                    wlength,
                    windex,
                    wvalue
                );

            let request = Request::parse(bmrequesttype, brequest, wvalue, windex, wlength)
                .expect("Error parsing request");
            match request {
                Request::GetDescriptor {
                    descriptor: Descriptor::Device,
                    length,
                } => {
                    defmt::info!("GET_DESCRIPTOR Device [length={}]", length);

                    defmt::println!("Goal reached; move to the next section");
                    dk::exit()
                }
                Request::SetAddress { .. } => {
                    // On macOS you'll get this request before the GET_DESCRIPTOR request so we
                    // need to catch it here. We'll properly handle this request later
                    // but for now it's OK to do nothing.
                }
                _ => unreachable!(), // we don't handle any other Requests
            }
        }
    }
}

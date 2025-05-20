#![no_main]
#![no_std]

use dk::{
    peripheral::USBD,
    usbd::{self, Ep0In, Event},
};

use usb::{Descriptor, Request};

// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use usb_app as _;

#[rtic::app(device = dk, peripherals = false)]
mod app {
    use super::*;

    #[local]
    struct MyLocalResources {
        usbd: USBD,
        ep0in: Ep0In,
    }

    #[shared]
    struct MySharedResources {}

    #[init]
    fn init(_cx: init::Context) -> (MySharedResources, MyLocalResources) {
        let board = dk::init().unwrap();

        usbd::init(board.power, &board.usbd);

        (
            MySharedResources {},
            MyLocalResources {
                usbd: board.usbd,
                ep0in: board.ep0in,
            },
        )
    }

    #[task(binds = USBD, local = [usbd, ep0in])]
    fn handle_usb_interrupt(cx: handle_usb_interrupt::Context) {
        while let Some(event) = usbd::next_event(cx.local.usbd) {
            on_event(cx.local.usbd, cx.local.ep0in, event)
        }
    }
}

/// Handle a USB event (in interrupt context)
fn on_event(usbd: &USBD, ep0in: &mut Ep0In, event: Event) {
    defmt::debug!("USB: {} @ {=u64:tus}", event, dk::uptime_us());

    match event {
        Event::UsbReset => {
            // nothing to do here at the moment
        }

        Event::UsbEp0DataDone => todo!(), // <- TODO

        Event::UsbEp0Setup => {
            let bmrequesttype = usbd::bmrequesttype(usbd);
            let brequest = usbd::brequest(usbd);
            let wlength = usbd::wlength(usbd);
            let windex = usbd::windex(usbd);
            let wvalue = usbd::wvalue(usbd);

            defmt::debug!(
                    "SETUP: bmrequesttype: 0b{=u8:08b}, brequest: {=u8}, wlength: {=u16}, windex: 0x{=u16:04x}, wvalue: 0x{=u16:04x}",
                    bmrequesttype,
                    brequest,
                    wlength,
                    windex,
                    wvalue
                );

            let request = Request::parse(bmrequesttype, brequest, wvalue, windex, wlength).expect(
                "Error parsing request (goal achieved if GET_DESCRIPTOR Device was handled before)",
            );
            match request {
                Request::GetDescriptor { descriptor, length }
                    if descriptor == Descriptor::Device =>
                {
                    defmt::info!("GET_DESCRIPTOR Device [length={}]", length);

                    // TODO send back a valid device descriptor, truncated to `length` bytes
                    // let desc = usb2::device::Descriptor { .. };
                    let resp = [];
                    ep0in.start(&resp, usbd);
                }
                Request::SetAddress { .. } => {
                    // On macOS you'll get this request before the GET_DESCRIPTOR request so we
                    // need to catch it here. We'll properly handle this request later
                    // but for now it's OK to do nothing.
                }
                _ => {
                    defmt::error!(
                            "unknown request (goal achieved if GET_DESCRIPTOR Device was handled before)"
                        );
                    dk::exit()
                }
            }
        }
    }
}

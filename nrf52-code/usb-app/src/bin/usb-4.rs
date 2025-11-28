#![no_main]
#![no_std]

use dk::hal::pac::usbd::Usbd;
use dk::usbd::{self, Ep0In, Event};

use usb2::State;

// HEADS UP to use *your* USB packet parser swap the two lines below
// use usb::{Request, Descriptor};
use usb2::{GetDescriptor as Descriptor, StandardRequest as Request};

// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use usb_app as _;

#[rtic::app(device = dk, peripherals = false)]
mod app {

    use super::*;

    #[local]
    struct MyLocalResources {
        usbd: Usbd,
        ep0in: Ep0In,
        state: State,
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
                state: State::Default,
            },
        )
    }

    #[task(binds = USBD, local = [usbd, ep0in, state])]
    fn handle_usb_interrupt(cx: handle_usb_interrupt::Context) {
        while let Some(event) = usbd::next_event(cx.local.usbd) {
            on_event(cx.local.usbd, cx.local.ep0in, cx.local.state, event)
        }
    }
}

/// Handle a USB event (in interrupt context)
fn on_event(usbd: &Usbd, ep0in: &mut Ep0In, state: &mut State, event: Event) {
    defmt::debug!("USB: {} @ {=u64:tus}", event, dk::uptime_us());

    match event {
        // TODO change `state` as specified in chapter 9.1 USB Device States, of the USB specification
        Event::UsbReset => {
            defmt::warn!("USB reset condition detected");
            todo!();
        }

        Event::UsbEp0DataDone => {
            defmt::info!("EP0IN: transfer complete");
            ep0in.end(usbd);
        }

        Event::UsbEp0Setup => {
            if ep0setup(usbd, ep0in, state).is_err() {
                // unsupported or invalid request:
                // TODO: add code to stall the endpoint
                defmt::warn!("EP0IN: unexpected request; stalling the endpoint");
            }
        }
    }
}

/// Handle a SETUP request on EP0
fn ep0setup(usbd: &Usbd, ep0in: &mut Ep0In, _state: &mut State) -> Result<(), ()> {
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

    let request = Request::parse(bmrequesttype, brequest, wvalue, windex, wlength);
    defmt::info!("EP0: {}", defmt::Debug2Format(&request));
    //                      ^^^^^^^^^^^^^^^^^^^ this adapter is currently needed to log
    //                                          `Request` with `defmt`
    match request {
        Ok(Request::GetDescriptor {
            descriptor: Descriptor::Device,
            length,
        }) => {
            let desc = usb2::device::Descriptor {
                bDeviceClass: 0,
                bDeviceProtocol: 0,
                bDeviceSubClass: 0,
                bMaxPacketSize0: usb2::device::bMaxPacketSize0::B64,
                bNumConfigurations: core::num::NonZeroU8::new(1).unwrap(),
                bcdDevice: 0x01_00, // 1.00
                iManufacturer: None,
                iProduct: None,
                iSerialNumber: None,
                idProduct: consts::USB_PID_RTIC_DEMO,
                idVendor: consts::USB_VID_DEMO,
            };
            let length = usize::from(length);
            let desc_bytes = desc.bytes();
            let subslice = if length >= desc_bytes.len() {
                // they want all of it
                &desc_bytes[0..]
            } else {
                // they don't want all of it
                &desc_bytes[0..length]
            };
            ep0in.start(subslice, usbd);
        }
        // TODO implement Configuration descriptor
        Ok(Request::GetDescriptor {
            // descriptor: Descriptor::Configuration { index },
            descriptor: _,
            length: _,
        }) => {
            return Err(());
        }
        Ok(Request::SetAddress { .. }) => {
            // On macOS you'll get this request before the GET_DESCRIPTOR request so we
            // need to catch it here.

            // TODO: handle this request properly now.
            todo!()
        }

        Ok(_) => {
            // stall anything else
            return Err(());
        }
        Err(_) => {
            defmt::error!(
                "Failed to parse: bmrequesttype: 0b{=u8:08b}, brequest: {=u8}, wlength: {=u16}, windex: 0x{=u16:04x}, wvalue: 0x{=u16:04x}",
                bmrequesttype,
                brequest,
                wlength,
                windex,
                wvalue
            );
            return Err(());
        }
    }

    Ok(())
}

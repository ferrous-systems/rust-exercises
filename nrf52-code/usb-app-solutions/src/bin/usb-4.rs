#![no_main]
#![no_std]

use core::num::NonZeroU8;

use dk::{
    peripheral::USBD,
    usbd::{self, Ep0In, Event},
};

use usb2::{GetDescriptor as Descriptor, StandardRequest as Request, State};

// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use usb_app as _;

/// The `bConfigurationValue` of the only supported configuration
const CONFIG_VAL: u8 = 42;

#[rtic::app(device = dk, peripherals = false)]
mod app {
    use super::*;

    #[local]
    struct MyLocalResources {
        usbd: USBD,
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
fn on_event(usbd: &USBD, ep0in: &mut Ep0In, state: &mut State, event: Event) {
    defmt::debug!("USB: {} @ {=u64:tus}", event, dk::uptime_us());

    match event {
        Event::UsbReset => {
            defmt::warn!("USB reset condition detected");
            *state = State::Default;
        }

        Event::UsbEp0DataDone => {
            defmt::info!("EP0IN: transfer complete");
            ep0in.end(usbd);
        }

        Event::UsbEp0Setup => {
            if ep0setup(usbd, ep0in, state).is_err() {
                defmt::warn!("EP0IN: unexpected request; stalling the endpoint");
                usbd::ep0stall(usbd);
            }
        }
    }
}

/// Handle a SETUP request on EP0
fn ep0setup(usbd: &USBD, ep0in: &mut Ep0In, state: &mut State) -> Result<(), ()> {
    let bmrequesttype = usbd.bmrequesttype.read().bits() as u8;
    let brequest = usbd.brequest.read().brequest().bits();
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

    let request = Request::parse(bmrequesttype, brequest, wvalue, windex, wlength)
        .expect("Error parsing request");
    defmt::info!("EP0: {}", defmt::Debug2Format(&request));
    //                         ^^^^^^^^^^^^^^^^^^^ this adapter is currently needed to log
    //                                            `Request` with `defmt`
    match request {
        // section 9.4.3
        // this request is valid in any state
        Request::GetDescriptor {
            descriptor: Descriptor::Device,
            length,
        } => {
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
        Request::GetDescriptor {
            descriptor: Descriptor::Configuration { index },
            length,
        } => {
            if index == 0 {
                let mut resp = heapless::Vec::<u8, 64>::new();

                let conf_desc = usb2::configuration::Descriptor {
                    wTotalLength: (usb2::configuration::Descriptor::SIZE
                        + usb2::interface::Descriptor::SIZE)
                        .into(),
                    bNumInterfaces: NonZeroU8::new(1).unwrap(),
                    bConfigurationValue: core::num::NonZeroU8::new(CONFIG_VAL).unwrap(),
                    iConfiguration: None,
                    bmAttributes: usb2::configuration::bmAttributes {
                        self_powered: true,
                        remote_wakeup: false,
                    },
                    bMaxPower: 250, // 500 mA
                };

                let iface_desc = usb2::interface::Descriptor {
                    bInterfaceNumber: 0,
                    bAlternativeSetting: 0,
                    bNumEndpoints: 0,
                    bInterfaceClass: 0,
                    bInterfaceSubClass: 0,
                    bInterfaceProtocol: 0,
                    iInterface: None,
                };

                resp.extend_from_slice(&conf_desc.bytes()).unwrap();
                resp.extend_from_slice(&iface_desc.bytes()).unwrap();
                ep0in.start(&resp[..core::cmp::min(resp.len(), length.into())], usbd);
            } else {
                // out of bounds access: stall the endpoint
                return Err(());
            }
        }
        Request::SetAddress { address } => {
            match state {
                State::Default => {
                    if let Some(address) = address {
                        *state = State::Address(address);
                    } else {
                        // stay in the `Default` state
                    }
                }

                State::Address(..) => {
                    if let Some(address) = address {
                        // use the new address
                        *state = State::Address(address);
                    } else {
                        *state = State::Default;
                    }
                }

                // unspecified behavior
                State::Configured { .. } => return Err(()),
            }

            // the response to this request is handled in hardware
        }

        // stall any other request
        _ => return Err(()),
    }

    Ok(())
}

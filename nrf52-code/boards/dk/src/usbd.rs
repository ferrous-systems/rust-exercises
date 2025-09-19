//! USBD peripheral

use core::sync::atomic::{self, Ordering};

use grounded::uninit::GroundedArrayCell;
use hal::pac::{power::Power, usbd::Usbd};

use crate::errata;

/// Endpoint IN 0
pub struct Ep0In {
    buffer: &'static GroundedArrayCell<u8, 64>,
    busy: bool,
}

impl Ep0In {
    /// # Safety
    /// Must be created at most once (singleton)
    pub(crate) unsafe fn new(buffer: &'static GroundedArrayCell<u8, 64>) -> Self {
        Self {
            buffer,
            busy: false,
        }
    }

    /// Starts a data transfer over endpoint 0
    ///
    /// # Panics
    ///
    /// - This function panics if the last transfer was not finished by calling the `end` function
    /// - This function panics if `bytes` is larger than the maximum packet size (64 bytes)
    pub fn start(&mut self, bytes: &[u8], usbd: &Usbd) {
        let (buffer_ptr, buffer_len) = self.buffer.get_ptr_len();
        assert!(!self.busy, "EP0IN: last transfer has not completed");
        assert!(
            bytes.len() <= buffer_len,
            "EP0IN: multi-packet data transfers are not supported"
        );

        let n = bytes.len();
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), buffer_ptr, n);
        }

        // use a "shortcut" to issue a status stage after the data transfer is complete
        usbd.shorts().modify(|w| w.set_ep0datadone_ep0status(true));
        usbd.epin(0).maxcnt().write(|w| w.set_maxcnt(n as u8));
        usbd.epin(0).ptr().write(|w| *w = buffer_ptr as u32);

        self.busy = true;

        defmt::debug!("EP0IN: start {}B transfer", n);

        // start DMA transfer
        dma_start();
        usbd.tasks_startepin(0).write(|w| *w = 1);
    }

    /// Completes a data transfer
    ///
    /// This function must be called after the EP0DATADONE event is raised
    ///
    /// # Panics
    ///
    /// This function panics if called before `start` or before the EP0DATADONE event is raised by
    /// the hardware
    pub fn end(&mut self, usbd: &Usbd) {
        if usbd.events_ep0datadone().read() == 0 {
            panic!("Ep0In::end called before the EP0DATADONE event was raised");
        } else {
            // DMA transfer complete
            dma_end();
            usbd.events_ep0datadone().write(|w| *w = 0);

            self.busy = false;
            defmt::info!("EP0IN: transfer done");
        }
    }
}

// memory barrier to synchronize the start of a DMA transfer (which will run in parallel) with the
// caller's memory operations
//
// This function call *must* be *followed* by a memory *store* operation. Memory operations that
// *precede* this function call will *not* be moved, by the compiler or the instruction pipeline, to
// *after* the function call.
fn dma_start() {
    atomic::fence(Ordering::Release);
}

// memory barrier to synchronize the end of a DMA transfer (which ran in parallel) to the caller's
// memory operations
//
// This function call *must* be *preceded* by a memory *load* operation. Memory operations that
// *follow* this function call will *not* be moved, by the compiler or the instruction pipeline, to
// *before* the function call.
fn dma_end() {
    atomic::fence(Ordering::Acquire);
}

/// Initializes the USBD peripheral
// NOTE will be called from user code; at that point the high frequency clock source has already
// been configured to use to the external crystal
// Reference: section 6.35.4 of the nRF52840 Product Specification
pub fn init(power: Power, usbd: &Usbd) {
    let mut once = true;

    // wait until the USB cable has been connected
    while power.events_usbdetected().read() == 0 {
        if once {
            defmt::info!("waiting for USB connection on port J3");
            once = false;
        }

        continue;
    }
    power.events_usbdetected().write_value(0);

    // workaround silicon bug
    unsafe { errata::e187a() }
    // enable the USB peripheral
    usbd.enable().write(|w| w.set_enable(true));

    // wait for the peripheral to signal it has reached the READY state
    while !usbd.eventcause().read().ready() {
        continue;
    }
    // setting the bit clears the flag
    usbd.eventcause().write(|w| w.set_ready(true));

    // if EVENTCAUSE is all zeroes then also clear the USBEVENT register
    if usbd.eventcause().read().0 == 0 {
        usbd.events_usbevent().write_value(0);
    }

    // complete the silicon bug workaround
    unsafe { errata::e187b() }

    // also need to wait for the USB power supply regulator to stabilize
    while power.events_usbpwrrdy().read() == 0 {
        continue;
    }
    power.events_usbpwrrdy().write_value(0);

    // before returning unmask the relevant interrupts
    usbd.intenset().write(|w| {
        w.set_ep0datadone(true);
        w.set_ep0setup(true);
        w.set_usbreset(true);
    });

    // enable the D+ line pull-up
    usbd.usbpullup().write(|w| w.set_connect(true));
}

/// Stalls endpoint 0
pub fn ep0stall(usbd: &Usbd) {
    usbd.tasks_ep0stall().write_value(1);
}

/// USBD.EVENTS registers mapped to an enum
#[derive(Debug, defmt::Format)]
pub enum Event {
    /// `EVENTS_USBRESET` register was active
    UsbReset,

    /// `EVENTS_EP0DATADONE` register was active
    UsbEp0DataDone,

    /// `EVENTS_EP0SETUP` register was active
    UsbEp0Setup,
}

/// Returns the next unhandled USB event; returns none if there's no event to handle
///
/// NOTE this function will clear the corresponding the EVENT register (*) so the caller should
/// handle the returned event properly. Expect for USBEVENT and EP0DATADONE
pub fn next_event(usbd: &Usbd) -> Option<Event> {
    if usbd.events_usbreset().read() != 0 {
        usbd.events_usbreset().write_value(0);

        return Some(Event::UsbReset);
    }

    if usbd.events_ep0datadone().read() != 0 {
        // this will be cleared by the `Ep0In::end` method
        // usbd.events_ep0datadone.reset();

        return Some(Event::UsbEp0DataDone);
    }

    if usbd.events_ep0setup().read() != 0 {
        usbd.events_ep0setup().write_value(0);

        return Some(Event::UsbEp0Setup);
    }

    None
}

/// Reads the BMREQUESTTYPE register and returns the 8-bit BMREQUESTTYPE component of a setup packet
pub fn bmrequesttype(usbd: &Usbd) -> u8 {
    // read the 32-bit register and extract the least significant byte
    // (the alternative is to read the 3 bitfields of the register and merge them into one byte)
    usbd.bmrequesttype().read().0 as u8
}

/// Reads the BREQUEST register and returns the 8-bit BREQUEST component of a setup packet
pub fn brequest(usbd: &Usbd) -> u8 {
    usbd.brequest().read().brequest() as u8
}

/// Reads the WLENGTHL and WLENGTHH registers and returns the 16-bit WLENGTH component of a setup packet
pub fn wlength(usbd: &Usbd) -> u16 {
    u16::from(usbd.wlengthl().read().wlengthl()) | u16::from(usbd.wlengthh().read().wlengthh()) << 8
}

/// Reads the WINDEXL and WINDEXH registers and returns the 16-bit WINDEX component of a setup packet
pub fn windex(usbd: &Usbd) -> u16 {
    u16::from(usbd.windexl().read().windexl()) | u16::from(usbd.windexh().read().windexh()) << 8
}

/// Reads the WVALUEL and WVALUEH registers and returns the 16-bit WVALUE component of a setup packet
pub fn wvalue(usbd: &Usbd) -> u16 {
    u16::from(usbd.wvaluel().read().wvaluel()) | u16::from(usbd.wvalueh().read().wvalueh()) << 8
}

//! USBD peripheral

use core::sync::atomic::{self, Ordering};

use crate::{
    errata,
    peripheral::{POWER, USBD},
};

/// Endpoint IN 0
pub struct Ep0In {
    buffer: &'static mut [u8; 64],
    busy: bool,
}

impl Ep0In {
    /// # Safety
    /// Must be created at most once (singleton)
    pub(crate) unsafe fn new(buffer: &'static mut [u8; 64]) -> Self {
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
    pub fn start(&mut self, bytes: &[u8], usbd: &USBD) {
        assert!(!self.busy, "EP0IN: last transfer has not completed");
        assert!(
            bytes.len() <= self.buffer.len(),
            "EP0IN: multi-packet data transfers are not supported"
        );

        let n = bytes.len();
        self.buffer[..n].copy_from_slice(bytes);

        // use a "shortcut" to issue a status stage after the data transfer is complete
        usbd.shorts
            .modify(|_, w| w.ep0datadone_ep0status().set_bit());
        usbd.epin0
            .maxcnt
            .write(|w| unsafe { w.maxcnt().bits(n as u8) });
        usbd.epin0
            .ptr
            .write(|w| unsafe { w.ptr().bits(self.buffer.as_ptr() as u32) });

        self.busy = true;

        defmt::println!("EP0IN: start {}B transfer", n);

        // start DMA transfer
        dma_start();
        usbd.tasks_startepin[0].write(|w| w.tasks_startepin().set_bit());
    }

    /// Completes a data transfer
    ///
    /// This function must be called after the EP0DATADONE event is raised
    ///
    /// # Panics
    ///
    /// This function panics if called before `start` or before the EP0DATADONE event is raised by
    /// the hardware
    pub fn end(&mut self, usbd: &USBD) {
        if usbd.events_ep0datadone.read().bits() == 0 {
            panic!("Ep0In.end called before the EP0DATADONE event was raised");
        } else {
            // DMA transfer complete
            dma_end();
            usbd.events_ep0datadone.reset();

            self.busy = false;
            defmt::println!("EP0IN: transfer done");
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
pub fn init(power: POWER, usbd: &USBD) {
    let mut once = true;

    // wait until the USB cable has been connected
    while power.events_usbdetected.read().bits() == 0 {
        if once {
            defmt::println!("waiting for USB connection on port J3");
            once = false;
        }

        continue;
    }
    power.events_usbdetected.reset();

    // workaround silicon bug
    unsafe { errata::e187a() }
    // enable the USB peripheral
    usbd.enable.write(|w| w.enable().set_bit());

    // wait for the peripheral to signal it has reached the READY state
    while usbd.eventcause.read().ready().bit_is_clear() {
        continue;
    }
    // write 1 to clear the flag
    usbd.eventcause.write(|w| w.ready().set_bit());

    // if EVENTCAUSE is all zeroes then also clear the USBEVENT register
    if usbd.eventcause.read().bits() == 0 {
        usbd.events_usbevent.reset();
    }

    // complete the silicon bug workaround
    unsafe { errata::e187b() }

    // also need to wait for the USB power supply regulator to stabilize
    while power.events_usbpwrrdy.read().bits() == 0 {
        continue;
    }
    power.events_usbpwrrdy.reset();

    // before returning unmask the relevant interrupts
    usbd.intenset.write(|w| {
        w.ep0datadone().set_bit();
        w.ep0setup().set_bit();
        w.usbreset().set_bit()
    });

    // enable the D+ line pull-up
    usbd.usbpullup.write(|w| w.connect().set_bit());
}

/// Stalls endpoint 0
pub fn ep0stall(usbd: &USBD) {
    usbd.tasks_ep0stall.write(|w| w.tasks_ep0stall().set_bit());
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
pub fn next_event(usbd: &USBD) -> Option<Event> {
    if usbd.events_usbreset.read().bits() != 0 {
        usbd.events_usbreset.reset();

        return Some(Event::UsbReset);
    }

    if usbd.events_ep0datadone.read().bits() != 0 {
        // this will be cleared by the `Ep0In.end` method
        // usbd.events_ep0datadone.reset();

        return Some(Event::UsbEp0DataDone);
    }

    if usbd.events_ep0setup.read().bits() != 0 {
        usbd.events_ep0setup.reset();

        return Some(Event::UsbEp0Setup);
    }

    None
}

/// Reads the BMREQUESTTYPE register and returns the 8-bit BMREQUESTTYPE component of a setup packet
pub fn bmrequesttype(usbd: &USBD) -> u8 {
    // read the 32-bit register and extract the least significant byte
    // (the alternative is to read the 3 bitfields of the register and merge them into one byte)
    usbd.bmrequesttype.read().bits() as u8
}

/// Reads the BREQUEST register and returns the 8-bit BREQUEST component of a setup packet
pub fn brequest(usbd: &USBD) -> u8 {
    usbd.brequest.read().brequest().bits()
}

/// Reads the WLENGTHL and WLENGTHH registers and returns the 16-bit WLENGTH component of a setup packet
pub fn wlength(usbd: &USBD) -> u16 {
    u16::from(usbd.wlengthl.read().wlengthl().bits())
        | u16::from(usbd.wlengthh.read().wlengthh().bits()) << 8
}

/// Reads the WINDEXL and WINDEXH registers and returns the 16-bit WINDEX component of a setup packet
pub fn windex(usbd: &USBD) -> u16 {
    u16::from(usbd.windexl.read().windexl().bits())
        | u16::from(usbd.windexh.read().windexh().bits()) << 8
}

/// Reads the WVALUEL and WVALUEH registers and returns the 16-bit WVALUE component of a setup packet
pub fn wvalue(usbd: &USBD) -> u16 {
    u16::from(usbd.wvaluel.read().wvaluel().bits())
        | u16::from(usbd.wvalueh.read().wvalueh().bits()) << 8
}

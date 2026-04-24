#![no_std]

/// A USB VID we randomly picked for the demo code on the dongle
pub const USB_VID_DEMO: u16 = 0x1209;

/// USB PID for the nRF52840 in USB mode when running the RTIC demo
pub const USB_PID_RTIC_DEMO: u16 = 0x0001;

/// USB PID for the Dongle in Puzzle mode
pub const USB_PID_DONGLE_UNIFIED: u16 = 0x0003;

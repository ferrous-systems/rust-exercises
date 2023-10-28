#![no_std]

/// A USB VID we randomly picked for the demo code on the dongle
pub const USB_VID_DEMO: u16 = 0x1209;

/// USB PID for the nRF52840 in USB mode when running the RTIC demo
pub const USB_PID_RTIC_DEMO: u16 = 0x0717;

/// USB PID for the Dongle in Loopback mode
pub const USB_PID_DONGLE_LOOPBACK: u16 = 0x0309;

/// USB PID for the Dongle in Puzzle mode
pub const USB_PID_DONGLE_PUZZLE: u16 = 0x0310;

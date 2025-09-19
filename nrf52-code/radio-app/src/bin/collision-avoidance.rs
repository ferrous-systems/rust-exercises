#![deny(unused_must_use)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![no_main]
#![no_std]

use core::{str, time::Duration};

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use dk::ieee802154::{Cca, Channel, Packet, Radio, TxPower};
// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use radio_app as _;

// You might only want to do this once.
const SCAN_ENERGIES: bool = true;

// This might simplify iteration over all channels.
static CHANNEL_SLICE: &[Channel] = &[
    Channel::_11,
    Channel::_12,
    Channel::_13,
    Channel::_14,
    Channel::_15,
    Channel::_16,
    Channel::_17,
    Channel::_18,
    Channel::_19,
    Channel::_20,
    Channel::_21,
    Channel::_22,
    Channel::_23,
    Channel::_24,
    Channel::_25,
    Channel::_26,
];

#[entry]
fn main() -> ! {
    let board = dk::init().unwrap();
    let mut _radio = board.radio;
    let mut _timer = board.timer;

    if SCAN_ENERGIES {
        // Put your code to scan all energy levels here.
    }

    // these three are equivalent
    let msg: &[u8] = b"Hello, this is a large test string";

    let mut packet = Packet::new();

    defmt::println!(
        "sending: {}",
        str::from_utf8(msg).expect("msg is not valid UTF-8 data")
    );

    packet.copy_from_slice(msg);

    // You might have to set CCA and channel before your loop logic.

    loop {
        // Put your application logic here.
        nop();
    }
}

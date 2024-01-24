#![deny(unused_must_use)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use dk::ieee802154::{Channel, Packet};
use heapless::LinearMap;
// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use radio_app as _;

const TEN_MS: u32 = 10_000;

#[entry]
fn main() -> ! {
    let board = dk::init().unwrap();
    let mut radio = board.radio;
    let mut timer = board.timer;

    // puzzle-fw uses channel 25 by default
    // NOTE if you ran `change-channel` then you may need to update the channel here
    radio.set_channel(Channel::_25); // <- must match the Dongle's listening channel

    // capacity (128) should be large enough for the ASCII range
    let dict = LinearMap::<u8, u8, 128>::new();

    let mut packet = Packet::new();
    // TODO do the whole ASCII range [0, 127]
    for source in b'A'..=b'B' {
        packet.copy_from_slice(&[source]);

        radio.send(&mut packet);

        if radio.recv_timeout(&mut packet, &mut timer, TEN_MS).is_ok() {
            // response should be one byte large
            if packet.len() == 1 {
                let _destination = packet[0];

            // TODO insert the key-value pair
            // dict.insert(/* ? */, /* ? */).expect("dictionary full");
            } else {
                defmt::error!("response packet was not a single byte");
                dk::exit()
            }
        } else {
            defmt::error!("no response or response packet was corrupted");
            dk::exit()
        }
    }

    defmt::println!("{:?}", defmt::Debug2Format(&dict));
    //                   ^^^^^^^^^^^^^^^^^^^ this adapter is currently needed to log `heapless`
    //                                       data structures (like `LinearMap` here) with `defmt`

    dk::exit()
}

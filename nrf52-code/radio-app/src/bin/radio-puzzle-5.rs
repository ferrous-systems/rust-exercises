#![deny(unused_must_use)]
#![no_main]
#![no_std]

use core::str;

use cortex_m_rt::entry;
use dk::ieee802154::{Channel, Packet};
use heapless::Vec;
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

    let mut packet = Packet::new();

    /* # Retrieve the secret string */
    let Ok(secret) = dk::send_recv(&mut packet, &[], &mut radio, &mut timer, TEN_MS) else {
        defmt::error!("no response or response packet was corrupted");
        dk::exit()
    };

    defmt::println!(
        "ciphertext: {}",
        str::from_utf8(secret).expect("packet was not valid UTF-8")
    );

    /* # Decrypt the string */
    let mut buf = Vec::<u8, 128>::new();

    // iterate over the bytes
    for input in secret.iter() {
        // process each byte
        // here we should do the reverse mapping; instead we'll do a shift for illustrative purposes
        let output = input + 1;
        buf.push(output).expect("buffer full");
    }

    defmt::println!(
        "plaintext: {}",
        str::from_utf8(&buf).expect("buffer contains non-UTF-8 data")
    );

    dk::exit()
}

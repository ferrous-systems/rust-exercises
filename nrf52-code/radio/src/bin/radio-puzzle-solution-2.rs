#![deny(unused_must_use)]
#![no_main]
#![no_std]

use core::str;

use cortex_m_rt::entry;
use dk::ieee802154::{Channel, Packet};
use heapless::LinearMap;
// this imports `radio/src/lib.rs.rs` to retrieve our global logger + panicking-behavior
use radio as _;

const TEN_MS: u32 = 10_000;

#[entry]
fn main() -> ! {
    let board = dk::init().unwrap();
    let mut radio = board.radio;
    let mut timer = board.timer;

    // puzzle.hex uses channel 25
    radio.set_channel(Channel::_25);

    /* # Build a dictionary */
    let mut dict = LinearMap::<u8, u8, 128>::new();

    // the IEEE 802.15.4 packet that will carry our data
    let mut packet = Packet::new();
    for plainletter in 0..=127 {
        packet.copy_from_slice(&[plainletter]);

        radio.send(&mut packet);

        if radio.recv_timeout(&mut packet, &mut timer, TEN_MS).is_ok() {
            // response should be one byte large
            if packet.len() == 1 {
                let cipherletter = packet[0];

                dict.insert(cipherletter, plainletter)
                    .expect("dictionary full");
            } else {
                defmt::error!("response packet was not a single byte");
                dk::exit()
            }
        } else {
            defmt::error!("no response or response packet was corrupted");
            dk::exit()
        }
    }

    /* # Retrieve the secret string */
    packet.copy_from_slice(&[]); // empty packet
    radio.send(&mut packet);

    if radio.recv_timeout(&mut packet, &mut timer, TEN_MS).is_err() {
        defmt::error!("no response or response packet was corrupted");
        dk::exit()
    }

    defmt::println!(
        "ciphertext: {}",
        str::from_utf8(&packet).expect("packet was not valid UTF-8")
    );

    /* # Decrypt the string *in place* */
    // NOTE *mutably* iterate over the bytes
    for spot in packet.iter_mut() {
        // `spot` has type `&mut u8` and lets you modify the contents of the packet
        let cipherletter = *spot; // make a copy of the byte
        let key = cipherletter;
        let value = dict[&key];

        let plainletter = value;
        // overwrite the old value with the plainletter
        *spot = plainletter;
    }

    defmt::println!(
        "plaintext:  {}",
        str::from_utf8(&packet).expect("buffer contains non-UTF-8 data")
    );

    /* # Verify decrypted text */
    radio.send(&mut packet);

    if radio.recv_timeout(&mut packet, &mut timer, TEN_MS).is_err() {
        defmt::error!("no response or response packet was corrupted");
        dk::exit()
    }

    defmt::println!(
        "Dongle response: {}",
        str::from_utf8(&packet).expect("response was not UTF-8")
    );

    dk::exit()
}

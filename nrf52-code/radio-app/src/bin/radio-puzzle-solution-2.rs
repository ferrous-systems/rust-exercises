#![deny(unused_must_use)]
#![no_main]
#![no_std]

use core::str;

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

    // puzzle.hex uses channel 25 by default
    // NOTE if you ran `change-channel` then you may need to update the channel here
    radio.set_channel(Channel::_25); // <- must match the Dongle's listening channel

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
                                  // overwrite the old value with the plainletter
        *spot = dict[&cipherletter];
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

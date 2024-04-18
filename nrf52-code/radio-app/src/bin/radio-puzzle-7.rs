#![deny(unused_must_use)]
#![no_main]
#![no_std]

use core::str;

use cortex_m_rt::entry;
use dk::ieee802154::{Channel, Packet};
use heapless::{LinearMap, Vec};
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

    /* # Build a dictionary */
    let dict = LinearMap::<u8, u8, 128>::new();

    let mut packet = Packet::new();
    for input in 0..=127 {
        if let Ok(data) = dk::send_recv(&mut packet, &[input], &mut radio, &mut timer, TEN_MS) {
            // response should be one byte large
            if data.len() == 1 {
                let _output = data[0];
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

    /* # Retrieve the secret string */
    let Ok(secret) = dk::send_recv(&mut packet, &[], &mut radio, &mut timer, TEN_MS) else {
        defmt::error!("no response or response packet was corrupted");
        dk::exit()
    };

    defmt::println!(
        "ciphertext: {}",
        str::from_utf8(&secret).expect("packet was not valid UTF-8")
    );

    /* # Decrypt the string */
    let mut buffer = Vec::<u8, 128>::new();

    // iterate over the bytes
    for byte in packet.iter() {
        // NOTE this should map from the encrypted letter to the plaintext letter
        let key = byte;
        let value = dict[key];
        buffer.push(value).expect("buffer full");
    }

    defmt::println!(
        "plaintext: {}",
        str::from_utf8(&buffer).expect("buffer contains non-UTF-8 data")
    );

    /* # (NEW) Verify decrypted text */
    let Ok(response) = dk::send_recv(&mut packet, &buffer, &mut radio, &mut timer, TEN_MS) else {
        defmt::error!("no response or response packet was corrupted");
        dk::exit()
    };

    defmt::println!(
        "Dongle response: {}",
        str::from_utf8(&response).expect("response was not UTF-8")
    );

    dk::exit()
}

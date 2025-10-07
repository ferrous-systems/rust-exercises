#![deny(unused_must_use)]
#![no_main]
#![no_std]

use core::str;

use cortex_m_rt::entry;
use dk::radio::{Channel, Packet};
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
    let mut dict = LinearMap::<u8, u8, 128>::new();

    let mut packet = Packet::new();
    for input in 0..=127 {
        // send the plaintext
        if let Ok(data) = dk::send_recv(&mut packet, &[input], &mut radio, &mut timer, TEN_MS) {
            // response should be one byte large
            if data.len() == 1 {
                // get back the ciphertext, which we use as the key in our map
                let output = data[0];
                dict.insert(output, input).expect("dictionary full");
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
        str::from_utf8(secret).expect("packet was not valid UTF-8")
    );

    /* # Decrypt the string */
    let mut buffer = Vec::<u8, 128>::new();

    // iterate over the bytes
    for cipherletter in secret.iter() {
        let plainletter = dict[cipherletter];
        buffer.push(plainletter).expect("buffer full");
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
        str::from_utf8(response).expect("response was not UTF-8")
    );

    dk::exit()
}

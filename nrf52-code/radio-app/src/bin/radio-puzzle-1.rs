#![deny(unused_must_use)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use dk::ieee802154::{Channel, Packet};
// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use radio_app as _;

const TEN_MS: u32 = 10_000;

#[entry]
fn main() -> ! {
    let board = dk::init().unwrap();
    let mut radio = board.radio;
    let mut timer = board.timer;

    // puzzle.hex uses channel 25
    radio.set_channel(Channel::_25);

    // the IEEE 802.15.4 packet that will carry our data
    let mut packet = Packet::new();

    // first exchange a single packet with the Dongle
    // letter 'A' (uppercase)
    let source = 65;
    // let source = b'A'; // this is the same as above

    // TODO try other letters

    // single letter (byte) packet
    packet.copy_from_slice(&[source]);

    radio.send(&mut packet);

    if radio.recv_timeout(&mut packet, &mut timer, TEN_MS).is_ok() {
        // response should be one byte large
        if packet.len() == 1 {
            let destination = packet[0];

            defmt::println!("{} -> {}", source, destination);
            // or cast to `char` for a more readable output
            defmt::println!("{:?} -> {:?}", source as char, destination as char);
        } else {
            defmt::error!("response packet was not a single byte");
            dk::exit()
        }
    } else {
        defmt::error!("no response or response packet was corrupted");
        dk::exit()
    }

    // TODO next do the whole ASCII range [0, 127]
    // start small: just 'A' and 'B' at first
    // NOTE: `a..=b` means inclusive range; `a` and `b` are included in the range
    // `a..b` means open-ended range; `a` is included in the range but `b` isn't
    for _source in b'A'..=b'B' {
        // TODO similar procedure as above
    }

    dk::exit()
}

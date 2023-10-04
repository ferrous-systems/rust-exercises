#![deny(unused_must_use)]
#![no_main]
#![no_std]

use core::str;

use cortex_m_rt::entry;
use heapless::Vec;
// this imports `radio/src/lib.rs.rs` to retrieve our global logger + panicking-behavior
use radio as _;

#[entry]
fn main() -> ! {
    dk::init().unwrap();

    // a buffer with capacity for 2 bytes
    let mut buffer = Vec::<u8, 2>::new();
    //                content type ^^  ^ capacity

    // do some insertions
    buffer.push(b'H').expect("buffer full");
    buffer.push(b'i').expect("buffer full");

    // look into the contents so far
    defmt::println!("{}", defmt::Debug2Format(&buffer));
    //                         ^^^^^^^^^^^^^^^^^^^ this adapter is currently needed to log
    //                                            `StandardRequest` with `defmt`

    // or more readable
    // NOTE utf-8 conversion works as long as you only push bytes in the ASCII range (0..=127)
    defmt::println!(
        "{}",
        str::from_utf8(&buffer).expect("content was not UTF-8")
    );

    dk::exit()
}

#![deny(unused_must_use)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use heapless::Vec;
// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use usb_app as _;

#[entry]
fn main() -> ! {
    dk::init().unwrap();

    // a stack-allocated `Vec` with capacity for 6 bytes
    let mut buffer = Vec::<u8, 6>::new();
    //        content type ^^  ^ capacity

    // `heapless::Vec` exposes the same API surface as `std::Vec` but some of its methods return a
    // `Result` to indicate whether the operation failed due to the `heapless::Vec` being full
    defmt::println!("start: {:?}", buffer);

    buffer.push(0).expect("buffer full");
    defmt::println!("after `push`: {:?}", buffer);

    buffer.extend_from_slice(&[1, 2, 3]).expect("buffer full");
    defmt::println!("after `extend`: {:?}", &buffer);

    // TODO try this operation
    // buffer.extend_from_slice(&[4, 5, 6, 7]).expect("buffer full");

    // TODO try changing the capacity of the `heapless::Vec`

    dk::exit()
}

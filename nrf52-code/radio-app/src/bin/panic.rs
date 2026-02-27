#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use radio_app as _;

#[entry]
fn main() -> ! {
    dk::init().unwrap();

    // We purposely cause a panic here. Index has to be retrieved from a function, otherwise
    // Rust will actually catch the out-of-bounds error at compile time.
    let i = index();
    let array = [0, 1, 2];
    let x = array[i]; // out of bounds access
    defmt::println!("{}", x);

    loop {
        asm::bkpt();
    }
}

fn index() -> usize {
    3
}

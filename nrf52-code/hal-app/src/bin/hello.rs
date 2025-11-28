// this program does not use the standard library to avoid heap allocations.
// only the `core` library functions are available.
#![no_std]
// this program uses a custom entry point instead of `fn main()`
#![no_main]

use cortex_m_rt::entry;
// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use hal_app as _;

// the custom entry point
// 👇🏾
#[entry]
fn main() -> ! {
    //      ˆˆˆ
    //       ! is the 'never' type: this function never returns

    // initializes the peripherals
    dk::init();

    defmt::println!("Hello, world!"); // 👋🏾

    dk::exit();
}

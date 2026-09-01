//! # standalone-panic
//!
//! A basic standalone app showing how panic! works

// this program does not use the standard library to avoid heap allocations.
// only the `core` library functions are available.
#![no_std]
// this program uses a custom entry point instead of `fn main()`
#![no_main]

// We use defmt for logging output
use defmt_rtt as _;

// This is our Board Support Package, which we rename to something shorter
use nucleo_u5a5zj_bsp as bsp;

#[cortex_m_rt::entry]
fn main() -> ! {
    // this sets up all our hardware (we don't use the value we get back)
    _ = bsp::NonSecureBoard::new();

    defmt::println!("Hello, this is standalone-panic!");

    // We purposely cause a panic here. Index has to be retrieved from a function, otherwise
    // Rust will actually catch the out-of-bounds error at compile time.
    let i = index();
    let array = [0, 1, 2];
    let x = array[i]; // out of bounds access
    defmt::println!("x = {}", x);

    loop {}
}

fn index() -> usize {
    3
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", info);
    cortex_m::asm::bkpt();
    loop {}
}

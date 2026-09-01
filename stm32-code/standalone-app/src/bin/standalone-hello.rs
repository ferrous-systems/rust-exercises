//! # standalone-hello
//!
//! Prints hello using defmt

// this program does not use the standard library to avoid heap allocations.
// only the `core` library functions are available.
#![no_std]
// this program uses a custom entry point instead of `fn main()`
#![no_main]

// We use defmt for logging output
use defmt_rtt as _;

// This is our Board Support Package, which we need to mention
// so it actually gets linked in
use nucleo_u5a5zj_bsp as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!"); // 👋🏾

    loop {}
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    cortex_m::asm::bkpt();
    loop {}
}

//! # standalone-hello
//!
//! A complete Nonsecure State binary running on the NUCLEO-U5A5ZJ
//!
//! This program is linked to run at the start of Flash (the same place
//! `secure-loader` lives). If you already set TZEN=1 then don't worry,
//! this program also runs in secure state quite happily.

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

    defmt::println!("Hello, world!"); // 👋🏾

    loop {}
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", info);
    cortex_m::asm::bkpt();
    loop {}
}

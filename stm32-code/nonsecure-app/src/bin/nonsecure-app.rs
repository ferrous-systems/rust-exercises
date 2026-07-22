//! # nonsecure-app
//!
//! A skeleton template for a nonsecure app running on the NUCLEO-U5A5ZJ

#![no_std]
#![no_main]

use defmt_rtt as _;
use nucleo_u5a5zj_bsp as bsp;

#[cortex_m_rt::entry]
fn main() -> ! {
    let bsp = bsp::NonSecureBoard::new();
    defmt::info!("Hello, this is nonsecure-app!");

    for _ in 0..5 {
        defmt::info!("On...");
        bsp.green_ld1.on();
        cortex_m::asm::delay(1_000_000);

        defmt::info!("Off...");
        bsp.green_ld1.off();
        cortex_m::asm::delay(1_000_000);
    }

    loop {}
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", info);
    loop {}
}

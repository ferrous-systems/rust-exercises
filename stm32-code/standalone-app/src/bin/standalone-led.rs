//! # standalone-led
//!
//! Turns some LEDs on and off.

#![no_std]
#![no_main]

use defmt_rtt as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let board = nucleo_u5a5zj_bsp::NonSecureBoard::new();

    defmt::info!("Hello, this is standalone-led!");

    board.green_ld1.on();
    board.blue_ld2.on();
    board.red_ld3.off();

    // this program does not `exit`; use Ctrl+C to terminate it
    loop {}
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", info);
    cortex_m::asm::bkpt();
    loop {}
}

//! # standalone-blinky
//!
//! Blinks the LEDs

#![no_std]
#![no_main]

use defmt_rtt as _;
use nucleo_u5a5zj_bsp as bsp;

#[cortex_m_rt::entry]
fn main() -> ! {
    let board = bsp::NonSecureBoard::new();

    defmt::info!("Hello, this is standalone-blinky!");

    loop {
        defmt::info!("Green/Blue...");
        board.green_ld1.on();
        board.blue_ld2.on();
        board.red_ld3.off();
        cortex_m::asm::delay(1_000_000);

        defmt::info!("Red...");
        board.green_ld1.off();
        board.blue_ld2.off();
        board.red_ld3.on();
        cortex_m::asm::delay(1_000_000);
    }
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", info);
    cortex_m::asm::bkpt();
    loop {}
}

defmt::timestamp!("{=u32:tus}", bsp::timestamp());

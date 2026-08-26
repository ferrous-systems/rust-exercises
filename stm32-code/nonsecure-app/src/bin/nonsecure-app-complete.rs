//! # nonsecure-app, complete
//!
//! A complete Nonsecure State binary running on the NUCLEO-U5A5ZJ

#![no_std]
#![no_main]

use defmt_rtt as _;
use nucleo_u5a5zj_bsp as bsp;

unsafe extern "C" {
    safe fn secure_set_blue_led(value: u32);
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let board = bsp::NonSecureBoard::new();
    defmt::info!("Hello, this is nonsecure-app-complete!");

    for _ in 0..5 {
        defmt::info!("On...");
        board.green_ld1.on();
        secure_set_blue_led(0);
        cortex_m::asm::delay(1_000_000);

        defmt::info!("Off...");
        board.green_ld1.off();
        secure_set_blue_led(1);
        cortex_m::asm::delay(1_000_000);
    }

    defmt::println!("Being naughty and trying to read secure RAM...");

    let p = bsp::hal::ns_addr::SRAM1_START as *const u32;
    let value = unsafe { p.read() };

    defmt::println!("Read secure RAM? {}", value);

    loop {}
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", info);
    cortex_m::asm::bkpt();
    loop {}
}

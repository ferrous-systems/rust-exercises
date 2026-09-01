//! # standalone-button
//!
//! A starting point for the STM32 Buttons exercise.

#![no_std]
#![no_main]

use defmt_rtt as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let board = nucleo_u5a5zj_bsp::NonSecureBoard::new();

    defmt::info!("Hello, this is standalone-button!");

    board.green_ld1.off();
    board.blue_ld2.on();
    board.red_ld3.off();

    let mut was_pressed = false;
    loop {
        // This code will not compile until you've modified the BSP to add this API
        let button_pressed = false; // board.user_button.is_pressed();

        if button_pressed && !was_pressed {
            was_pressed = button_pressed;
            defmt::info!("Button Down");
            board.green_ld1.on();
            board.blue_ld2.off();
        } else if !button_pressed && was_pressed {
            was_pressed = button_pressed;
            defmt::info!("Button Up");
            board.green_ld1.off();
            board.blue_ld2.on();
        }
    }
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", info);
    cortex_m::asm::bkpt();
    loop {}
}

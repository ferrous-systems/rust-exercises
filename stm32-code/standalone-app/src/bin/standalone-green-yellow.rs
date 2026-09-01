//! # standalone-green-yellow
//!
//! A skeleton for the Green and Yellow game

#![no_std]
#![no_main]

use core::fmt::Write as _;
use defmt_rtt as _;
use nucleo_u5a5zj_bsp as bsp;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut board = bsp::NonSecureBoard::new();
    board.usart1.configure(bsp::APB2_PERIPH_CLK_HZ);

    _ = writeln!(board.usart1, "Welcome to the Green and Yellow game!");

    loop {
        let ch = board.usart1.rx_char_blocking();
        _ = writeln!(board.usart1, "You pressed 0x{:02x}", ch);
    }
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", info);
    cortex_m::asm::bkpt();
    loop {}
}

defmt::timestamp!("{=u32:tus}", bsp::timestamp());

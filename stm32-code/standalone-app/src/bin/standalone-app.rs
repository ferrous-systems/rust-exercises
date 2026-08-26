//! # standalone-app
//!
//! A complete Nonsecure State binary running on the NUCLEO-U5A5ZJ
//!
//! This program is linked to run at the start of Flash (the same place
//! `secure-loader` lives). If you already set TZEN=1 then don't worry,
//! this program also runs in secure state quite happily.

#![no_std]
#![no_main]

use core::fmt::Write as _;
use defmt_rtt as _;
use nucleo_u5a5zj_bsp as bsp;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut board = bsp::NonSecureBoard::new();
    board.usart1.configure(bsp::APB2_PERIPH_CLK_HZ);

    defmt::info!("Hello, this is standalone-app!");

    // Check if we are in secure mode by probing access permissions to a variable on our stack
    let mut x = 0;
    let tt = cortex_m::cmse::TestTarget::check(&raw mut x, cortex_m::cmse::AccessType::Current);
    defmt::info!("In secure state? {}", tt.secure());

    for i in 0u64.. {
        defmt::info!("On...");
        board.green_ld1.on();
        cortex_m::asm::delay(1_000_000);

        defmt::info!("Off...");
        board.green_ld1.off();
        cortex_m::asm::delay(1_000_000);

        _ = writeln!(board.usart1, "Hello {i}");
        if let Some(ch) = board.usart1.rx_char() {
            defmt::info!("Got {=u8:02x} from UART", ch);
        }
    }

    loop {}
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", info);
    loop {}
}

defmt::timestamp!("{=u32:tus}", bsp::timestamp());

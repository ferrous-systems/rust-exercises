//! # standalone-green-yellow-complete
//!
//! A functioning version of the green-yellow game, running standalone.
//!
//! Connect to the USB Virtual COM port at 9600 baud, 8N1

#![no_std]
#![no_main]

use core::fmt::Write as _;
use defmt_rtt as _;
use nucleo_u5a5zj_bsp as bsp;
use rand::prelude::*;

use green_yellow_solution::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut board = bsp::NonSecureBoard::new();
    board.usart1.configure(bsp::APB2_PERIPH_CLK_HZ);

    loop {
        let mut secret = [0u8; NUM_DIGITS];
        for digit in secret.iter_mut() {
            *digit = board.rng.random_range(1..=9);
        }

        defmt::debug!("The secret is {:?}", secret);
        _ = writeln!(board.usart1, "Let's play the Green and Yellow Game!");

        loop {
            let mut guess = [0u8; NUM_DIGITS];

            let mut i = 0;
            loop {
                _ = write!(board.usart1, "\nEnter digit {}: ", i);
                let ch = board.usart1.rx_char_blocking();
                match ch {
                    b'1'..=b'9' => {
                        guess[i] = ch - b'0';
                        board.usart1.tx_char_blocking(ch);
                        i += 1;
                        if i == NUM_DIGITS {
                            break;
                        }
                    }
                    _ => {
                        _ = writeln!(board.usart1, "{:?} is not valid, try again", ch as char);
                    }
                }
            }

            _ = writeln!(board.usart1, "\nYour guess is {:?}", guess);

            let score = calc_green_and_yellow(&guess, &secret);

            _ = writeln!(board.usart1, "That gives: {:?}", score);

            if guess == secret {
                _ = writeln!(board.usart1, "Well done!!");
                break;
            }
        }
    }
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", info);
    cortex_m::asm::bkpt();
    loop {}
}

defmt::timestamp!("{=u32:tus}", bsp::timestamp());

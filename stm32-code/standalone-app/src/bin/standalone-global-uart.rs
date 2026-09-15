//! # standalone-global-uart
//!
//! Sets up the UART as a global variable

#![no_std]
#![no_main]

use core::cell::RefCell;
use core::fmt::Write;

use critical_section::Mutex;
use defmt_rtt as _;

use nucleo_u5a5zj_bsp::{self as bsp, hal::usart::Usart1Driver, interrupt};

static GLOBAL_UART: Mutex<RefCell<Option<Usart1Driver>>> = Mutex::new(RefCell::new(None));

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut board = bsp::NonSecureBoard::new();
    board.usart1.configure(bsp::APB2_PERIPH_CLK_HZ);

    board.usart1.write_str("Hello on the UART").unwrap();

    // enable a reason to generate interrupt, in the UART peripheral
    board.usart1.rx_interrupt_enable(true);

    // move the UART driver into the global variable
    critical_section::with(|cs| {
        let mut uart_ref = GLOBAL_UART.borrow_ref_mut(cs);
        *uart_ref = Some(board.usart1);
    });

    // Safety: we are not in a critical section, so are safe to enable these interrupts
    unsafe {
        // enable the USART1 interrupt in the NVIC
        cortex_m::peripheral::NVIC::unmask(interrupt::USART1);
        // enable interrupts in the processor
        cortex_m::interrupt::enable();
    }

    do_stuff();

    loop {}
}

/// This is a freestanding function that is not given a reference to a UART driver
/// so locks and then uses the global UART to print some text
fn do_stuff() {
    // Note that the c-s is locked (i.e. interrupts are off) for
    // the whole duration of the "write_str" call
    critical_section::with(|cs| {
        // Lock the Mutex and mutably borrow the RefCell, in one go
        let mut uart = GLOBAL_UART.borrow_ref_mut(cs);
        // Only borrow the UART driver, don't move it!
        // `Option::as_mut` turns `&mut Option<T>` into `Option<&mut T>`
        if let Some(uart) = uart.as_mut() {
            uart.write_str("Hello on the UART").unwrap();
        }
    });
}

#[interrupt]
fn USART1() {
    critical_section::with(|cs| {
        // Lock the Mutex and mutably borrow the RefCell, in one go
        let mut uart = GLOBAL_UART.borrow_ref_mut(cs);
        // Again, only borrow the UART driver, don't move it!
        if let Some(uart) = uart.as_mut() {
            let byte = uart.rx_char_blocking();
            defmt::info!("Read byte {=u8:02x}", byte);
        }
        // TODO: check other reasons we might have gotten an interrupt
    });
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    cortex_m::asm::bkpt();
    loop {}
}

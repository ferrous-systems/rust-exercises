//! # standalone-panic
//!
//! A basic standalone app showing how panic! works

#![no_std]
#![no_main]

use defmt_rtt as _;
use nucleo_u5a5zj_bsp as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, this is standalone-app!");

    panic!("This is a sample panic");
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", info);
    loop {}
}

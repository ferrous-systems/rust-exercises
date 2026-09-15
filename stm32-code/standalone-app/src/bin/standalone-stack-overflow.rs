//! # standalone-stack-overflow
//!
//! Demonstrates a stack overflow. We should get a HardFault by
//! when MSP hits the MSPLIM value set by cortex-m-rt.

#![no_std]
#![no_main]

// We use defmt for logging output
use defmt_rtt as _;

// so it actually gets linked in
use nucleo_u5a5zj_bsp as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Starting infinite recursion...");

    loop {
        foo();
    }
}

fn foo() {
    let x = [0u8; 1024 * 32];
    bar((&raw const x) as usize);
}

fn bar(x: usize) {
    defmt::println!("addr = 0x{=usize:08x}", x);
    foo();
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    cortex_m::asm::bkpt();
    loop {}
}

#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use usb_app as _;

#[entry]
fn main() -> ! {
    // board initialization
    dk::init().unwrap();

    fib(100);

    loop {
        asm::bkpt();
    }
}

#[inline(never)]
fn fib(n: u32) -> u32 {
    // allocate and initialize one kilobyte of stack memory to provoke stack overflow
    let use_stack = [0xAA; 1024];
    defmt::println!("allocating [{}; 1024]; round #{}", use_stack[1023], n);

    if n < 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2) // recursion
    }
}

//! A uart-driver example program for QEMU's Armv8-R Virtual Machine
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2025

#![no_std]
#![no_main]

use core::fmt::Write;

use uart_exercise::PERIPHERAL_CLOCK;

// 👇 change over which driver is imported, so you can test your solution!
use uart_exercise::uart_driver::Uart;
// use uart_exercise::uart_driver_solution::Uart;

/// The entry-point to the Rust application.
///
/// It is called by the start-up assembly code in `cortex-r-rt` and thus
/// exported as a C-compatible symbol.
#[no_mangle]
pub extern "C" fn kmain() {
    if let Err(e) = main() {
        panic!("main returned {:?}", e);
    }
}

/// The main function of our Rust application.
///
/// Called by [`kmain`].
fn main() -> Result<(), core::fmt::Error> {
    semihosting::println!("Starting main...");
    let mut uart0 = unsafe { Uart::new_uart0() };
    uart0.enable(115200, PERIPHERAL_CLOCK);
    writeln!(uart0, "Hello, this is Rust!")?;
    // Print a multiplication square, using floating point
    for x in 1..=10 {
        for y in 1..=10 {
            let z = f64::from(x) * f64::from(y);
            write!(uart0, "{z:>8.2} ")?;
        }
        writeln!(uart0)?;
    }
    // Now exit the program
    semihosting::process::exit(0);
}

// End of file

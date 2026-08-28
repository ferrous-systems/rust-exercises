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
// use uart_exercise::uart_driver::Uart;
use uart_exercise::uart_driver_solution::Uart;

/// The entry-point to the Rust application.
///
/// It is called by the start-up assembly code in `aarch32-rt`.
#[aarch32_rt::entry]
fn main() -> ! {
    if let Err(e) = inner_main() {
        panic!("main returned {:?}", e);
    }
    semihosting::process::exit(0);
}

/// The main function of our Rust application.
///
/// Called by [`main`].
fn inner_main() -> Result<(), core::fmt::Error> {
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
    Ok(())
}

// End of file

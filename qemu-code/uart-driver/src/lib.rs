//! Support for the uart-driver example for QEMU's Armv8-R Virtual Machine
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2025

#![no_std]

pub mod uart_driver;
pub mod uart_driver_solution;

use cortex_r_rt as _;

/// The clock speed of the peripheral subsystem on an SSE-300 SoC an on MPS3 board.
///
/// Probably right for an MPS3-536
pub const PERIPHERAL_CLOCK: u32 = 25_000_000;

/// Called when the application raises an unrecoverable `panic!`.
///
/// Prints the panic to the console and then exits QEMU using a semihosting
/// breakpoint.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    semihosting::println!("PANIC: {:?}", info);
    semihosting::process::exit(1);
}

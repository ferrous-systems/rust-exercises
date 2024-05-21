//! Support for the uart-driver example for QEMU's Armv8-R Virtual Machine
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2024

#![no_std]

pub mod uart_driver;
pub mod uart_driver_solution;

use core::fmt::Write;

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
    const SYS_REPORTEXC: u32 = 0x18;
    // Forcibly re-init the UART driver, so panic has a chance of printing something
    let mut uart0 = unsafe { uart_driver_solution::Uart::new_uart0() };
    uart0.enable(115200, PERIPHERAL_CLOCK);
    let _ = writeln!(uart0, "PANIC: {:?}", info);
    loop {
        // Exit, using semihosting
        unsafe {
            core::arch::asm!(
                "svc 0x123456",
                in("r0") SYS_REPORTEXC,
                in("r1") 0x20026
            )
        }
    }
}

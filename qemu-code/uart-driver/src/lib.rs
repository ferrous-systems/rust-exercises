//! Support for the uart-driver example for QEMU's Armv8-R Virtual Machine
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2025

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

// This is our start-up code.
core::arch::global_asm!(r"
.section .text.startup
.global _start
.code 32
.align 0

_start:
    // Set stack pointer
    ldr r3, =stack_top
    mov sp, r3
    // Allow VFP coprocessor access
    mrc p15, 0, r0, c1, c0, 2
    orr r0, r0, #0xF00000
    mcr p15, 0, r0, c1, c0, 2
    // Enable VFP
    mov r0, #0x40000000
    vmsr fpexc, r0
    // Jump to application
    bl kmain
    // In case the application returns, loop forever
    b .
");

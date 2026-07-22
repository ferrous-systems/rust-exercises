//! # secure-loader, complete
//!
//! A Secure Mode binary, with the exercise completed.

#![no_std]
#![no_main]
#![feature(cmse_nonsecure_entry)]

use core::cell::RefCell;
use critical_section::Mutex;

use cortex_m_semihosting::hprintln;
use nucleo_u5a5zj_bsp as bsp;

/// The Red LED on the board
///
/// On when running. Blinks when a Secure Fault has occurred.
static RED_LED: Mutex<RefCell<Option<bsp::SecureLed>>> = Mutex::new(RefCell::new(None));

/// The Blue LED on the board
///
/// Controlled through a Secure Gateway function
static BLUE_LED: Mutex<RefCell<Option<bsp::SecureLed>>> = Mutex::new(RefCell::new(None));

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut bsp: bsp::SecureBoard = bsp::SecureBoard::new();

    // Enable secure fault handler
    bsp.scb
        .enable(cortex_m::peripheral::scb::Exception::SecureFault);

    // Say hello
    hprintln!("Hello, this is secure-loader. Configuring peripherals...");

    // Configure Secure Attribution Unit, to mark some addresses as Nonsecure
    bsp.configure_sau();
    hprintln!("...SAU configured");

    // Configure the Memory Protection Controller to mark all
    // the SRAM3 blocks as Nonsecure
    bsp.set_sram3_nonsecure();
    hprintln!("...GTZC1 configured");

    // We keep Red and Blue LEDs
    bsp.blue_ld2.off();
    bsp.red_ld3.on();
    // Save these for later
    critical_section::with(|cs| {
        BLUE_LED.replace(cs, Some(bsp.blue_ld2));
        RED_LED.replace(cs, Some(bsp.red_ld3));
    });

    // We give Green to Nonsecure Mode
    _ = bsp.green_ld1.make_nonsecure(&mut bsp.gpio);

    hprintln!("...LEDs configured");

    let ns_app_base = bsp::hal::ns_addr::FLASH2_START;
    hprintln!("Booting Nonsecure Mode binary at 0x{:08x}...", ns_app_base);
    // Boot a Nonsecure Mode binary
    unsafe {
        cortex_m::asm::bootload_ns(ns_app_base as *const u32, bsp.scb_ns);
    }
}

/// Control the blue LED
///
/// This function is exported and made available to the nonsecure side
#[unsafe(no_mangle)]
pub extern "cmse-nonsecure-entry" fn secure_set_blue_led(value: u32) {
    cortex_m_semihosting::hprintln!("secure_set_blue_led({})", value);
    critical_section::with(|cs| {
        if let Some(blue_led) = BLUE_LED.borrow_ref_mut(cs).as_mut() {
            if value == 0 {
                blue_led.off();
            } else {
                blue_led.on();
            }
        }
    });
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    hprintln!("PANIC: {}", info);
    loop {}
}

#[cortex_m_rt::exception]
fn SecureFault() {
    hprintln!("SECURE FAULT:");
    // Safety: No-one else is using the SAU, so this won't race, plus the registers we want are read-only
    let sau = unsafe { &*cortex_m::peripheral::SAU::PTR };
    let sfsr = sau.sfsr.read();
    if sfsr.sfarvalid() {
        hprintln!("- SFAR = {:08x}", sau.sfar.read().address());
    }
    if sfsr.invep() {
        hprintln!("- Invalid Entry Point");
    }
    if sfsr.invis() {
        hprintln!("- Invalid Integrity Signature");
    }
    if sfsr.inver() {
        hprintln!("- Invalid Exception Return");
    }
    if sfsr.auviol() {
        hprintln!("- Attribution Unit Violation");
    }
    if sfsr.invtran() {
        hprintln!("- Invalid Transition");
    }
    if sfsr.lsperr() {
        hprintln!("- Lazy state preservation error");
    }
    if sfsr.lserr() {
        hprintln!("- Lazy state error");
    }
    loop {
        critical_section::with(|cs| {
            if let Some(led) = RED_LED.borrow_ref_mut(cs).as_mut() {
                led.off();
            }
        });
        cortex_m::asm::delay(500_000);
        critical_section::with(|cs| {
            if let Some(led) = RED_LED.borrow_ref_mut(cs).as_mut() {
                led.on();
            }
        });
        cortex_m::asm::delay(500_000);
    }
}

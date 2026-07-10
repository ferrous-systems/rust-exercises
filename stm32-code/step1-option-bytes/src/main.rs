//! # step1-option-bytes
//!
//! A simple STM32 program for reprogramming its own option bytes

#![no_std]
#![no_main]

use cortex_m_semihosting::hprintln;
use stm32u5::stm32u5a5 as pac;

#[cortex_m_rt::entry]
fn main() -> ! {
    hprintln!("Running step1-option-bytes program.");

    let p = pac::Peripherals::take().unwrap();

    hprintln!("Enable FLASH peripheral...");
    p.RCC.ahb1enr().modify(|_r, w| {
        w.flashen().set_bit();
        w
    });
    let _ = p.RCC.ahb1enr().read();

    if p.FLASH.optr().read().tzen().bit_is_set() {
        hprintln!("TZEN=1 already. Doing nothing");
        loop {
            core::hint::spin_loop();
        }
    }
    hprintln!("Unlock FLASH_NSCR register...");
    p.FLASH.nskeyr().write(|w| {
        unsafe {
            w.bits(0x45670123);
        }
        w
    });
    p.FLASH.nskeyr().write(|w| {
        unsafe {
            w.bits(0xCDEF89AB);
        }
        w
    });

    hprintln!("Clear OPTLOCK...");
    // Wait for flash to not be busy
    while p.FLASH.nssr().read().bsy().bit_is_set() {
        core::hint::spin_loop();
    }
    p.FLASH.optkeyr().write(|w| {
        unsafe {
            w.optkey().bits(0x08192A3B);
        }
        w
    });
    p.FLASH.optkeyr().write(|w| {
        unsafe {
            w.optkey().bits(0x4C5D6E7F);
        }
        w
    });

    hprintln!("Set Option Bytes...");
    p.FLASH.optr().modify(|_r, w| {
        w.tzen().set_bit();
        w
    });

    hprintln!("Program Option Bytes...");
    p.FLASH.nscr().modify(|_r, w| {
        w.optstrt().set_bit();
        w
    });
    // Wait for flash to not be busy
    while p.FLASH.nssr().read().bsy().bit_is_set() {
        core::hint::spin_loop();
    }

    hprintln!("Reloading option Bytes. probe-rs is about to crash (and that's OK).");
    p.FLASH.nscr().modify(|_r, w| {
        w.obl_launch().set_bit();
        w
    });

    loop {}
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    hprintln!("PANIC: {}", info);
    loop {}
}

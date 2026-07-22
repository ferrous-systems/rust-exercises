//! # step2-secure-watermark
//!
//! A simple STM32 program for reprogramming its own secure watermark on Flash Bank 2.
//!
//! Requires that the chip is in Secure Mode (TZEN=1), so run step1-option-bytes first.

#![no_std]
#![no_main]

use cortex_m_semihosting::hprintln;
use stm32u5::stm32u5a5 as pac;

#[cortex_m_rt::entry]
fn main() -> ! {
    hprintln!("Running step2-secure-watermark program.");

    let p = pac::Peripherals::take().unwrap();

    hprintln!("Enable FLASH peripheral...");
    p.RCC.ahb1enr().modify(|_r, w| {
        w.flashen().set_bit();
        w
    });
    let _ = p.RCC.ahb1enr().read();

    if !p.SEC_FLASH.optr().read().tzen().bit_is_set() {
        hprintln!("TZEN=0?!?! Doing nothing");
        loop {
            core::hint::spin_loop();
        }
    }

    // Check if Bank 2 needs unprotecting (this will require a reboot)

    let secure_watermark2 = p.SEC_FLASH.secwm2r1().read();
    if secure_watermark2.secwm2_pstrt() == 0xFF && secure_watermark2.secwm2_pend() == 0 {
        hprintln!("Secure watermark is OK :)");
    } else {
        hprintln!("Unlocking Bank 2 from Secure Mode. probe-rs is about to crash and that's OK :)");
        // Unlock NSCR
        p.SEC_FLASH.nskeyr().write(|w| {
            unsafe {
                w.bits(0x45670123);
            }
            w
        });
        p.SEC_FLASH.nskeyr().write(|w| {
            unsafe {
                w.bits(0xCDEF89AB);
            }
            w
        });
        // Unlock Option Byte changes
        while p.SEC_FLASH.nssr().read().bsy().bit_is_set() {
            core::hint::spin_loop();
        }
        p.SEC_FLASH.optkeyr().write(|w| {
            unsafe {
                w.optkey().bits(0x08192A3B);
            }
            w
        });
        p.SEC_FLASH.optkeyr().write(|w| {
            unsafe {
                w.optkey().bits(0x4C5D6E7F);
            }
            w
        });
        // Mark Bank 2 as nonsecure by having pend < pstrt
        p.SEC_FLASH.secwm2r1().write(|w| {
            unsafe {
                w.secwm2_pend().bits(0x00);
                w.secwm2_pstrt().bits(0xFF);
            }
            w
        });
        // Start programming
        p.SEC_FLASH.nscr().modify(|_r, w| {
            w.optstrt().set_bit();
            w
        });
        while p.SEC_FLASH.nssr().read().bsy().bit_is_set() {
            core::hint::spin_loop();
        }
        // Reload settings
        p.SEC_FLASH.nscr().modify(|_r, w| {
            w.obl_launch().set_bit();
            w
        });
    }

    loop {}
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    hprintln!("PANIC: {}", info);
    loop {}
}

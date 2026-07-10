//! USART driver for STM32U5

use stm32u5::Periph;
use stm32u5::stm32u5a5 as pac;

/// A basic blocking USART driver
pub struct Driver<const ADDR: usize> {
    pac_object: Periph<pac::usart1::RegisterBlock, ADDR>,
}

impl<const ADDR: usize> Driver<ADDR> {
    /// Create a new USART driver, from a PAC object
    pub fn new(pac_object: Periph<pac::usart1::RegisterBlock, ADDR>) -> Self {
        Self { pac_object }
    }

    /// Configure the UART to 8N1, 9600 bps
    pub fn configure(&mut self) {
        // Calculate Baud Rate Register
        let brr = ((8_000_000u32 / 9600u32) / 8u32) as u16;
        // Disable UART
        self.pac_object.cr1().modify(|_r, w| {
            w.ue().clear_bit();
            w
        });
        // Configure UART
        self.pac_object.cr1().write(|w| {
            // FIFO Enabled
            w.fifoen().set_bit();
            // 16x oversampling
            w.over8().clear_bit();
            // Transmit Enabled
            w.te().set_bit();
            // Receive Disabled
            w.re().clear_bit();
            // 1 start bit, 8 data bits, N stop bits
            w.m0().clear_bit();
            w.m1().clear_bit();
            // No parity
            w.pce().clear_bit();
            w
        });
        self.pac_object.cr2().write(|w| {
            // 1 stop bit
            w.stop().stop1();
            w
        });
        self.pac_object.brr().write(|w| {
            w.brr().set(brr);
            w
        });
        // Enable UART
        self.pac_object.cr1().modify(|_r, w| {
            w.ue().set_bit();
            w
        });
    }

    /// Transmit a character
    pub fn tx_char_blocking(&mut self, ch: u8) {
        // wait for TX FIFO Not-Full to be set
        while self.pac_object.isr().read().txfnf().bit_is_clear() {
            core::hint::spin_loop();
        }
        // Transmit character
        self.pac_object.tdr().write(|w| {
            w.tdr().set(ch as u16);
            w
        });
    }
}

impl<const ADDR: usize> core::fmt::Write for Driver<ADDR> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            self.tx_char_blocking(b);
        }
        Ok(())
    }
}

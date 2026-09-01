//! USART driver for STM32U5

use stm32u5::Periph;
use stm32u5::stm32u5a5 as pac;

/// UART1, Secure State address
pub const USART1_S: usize = 0x5001_3800;

/// UART1, Nonsecure State address
pub const USART1_NS: usize = 0x4001_3800;

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
    pub fn configure(&mut self, apb_periph_clk_hz: u32) {
        // Calculate Baud Rate Register
        //
        // We have no UART prescaler, so we just have our 4 MHz system clock,
        // and we want a baud rate of 9600.
        //
        // The /2 is for rounding.
        let baud = 9600u32;
        let brr = ((apb_periph_clk_hz + (baud / 2)) / baud) as u16;
        // Disable UART
        self.pac_object.cr1().modify(|_r, w| {
            w.ue().clear_bit();
            w
        });
        // Configure UART
        self.pac_object.cr1().modify(|_r, w| {
            // FIFO Enabled
            w.fifoen().set_bit();
            // 16x oversampling
            w.over8().clear_bit();
            // Transmit Enabled
            w.te().set_bit();
            // Receive Enabled
            w.re().set_bit();
            // 1 start bit, 8 data bits
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
            // baud rate is as calculated previously
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

    /// Get a character, if one is waiting
    pub fn rx_char(&mut self) -> Option<u8> {
        if self.pac_object.isr().read().rxfne().bit_is_set() {
            // RX FIFO is Not Empty, so read it
            let byte = self.pac_object.rdr().read().bits() as u8;
            Some(byte)
        } else {
            // RX FIFO is Empty
            None
        }
    }

    /// Wait for a character and return it
    pub fn rx_char_blocking(&mut self) -> u8 {
        loop {
            if let Some(ch) = self.rx_char() {
                return ch;
            }
        }
    }

    /// Set whether the RX Interrupt is enabled
    pub fn rx_interrupt_enable(&mut self, enabled: bool) {
        self.pac_object.cr1().modify(|_r, w| {
            w.rxneie().bit(enabled);
            w
        });
    }
}

impl<const ADDR: usize> core::fmt::Write for Driver<ADDR> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            // convert LF to CRLF because Rust uses LF line endings but most terminals want CRLF
            if b == b'\n' {
                self.tx_char_blocking(b'\r');
            }
            self.tx_char_blocking(b);
        }
        Ok(())
    }
}

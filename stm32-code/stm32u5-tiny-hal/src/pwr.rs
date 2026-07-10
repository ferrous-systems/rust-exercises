//! PWR driver for STM32U5

use stm32u5::{Periph, stm32u5a5 as pac};

/// A basic PWR driver
pub struct Driver<const ADDR: usize> {
    pac_object: Periph<pac::pwr::RegisterBlock, ADDR>,
}

impl<const ADDR: usize> Driver<ADDR> {
    /// Create a new PWR driver, from a PAC object
    pub fn new(pac_object: Periph<pac::pwr::RegisterBlock, ADDR>) -> Self {
        Self { pac_object }
    }

    pub fn vddio2_enable(&mut self, enabled: bool) {
        self.pac_object.svmcr().modify(|_r, w| {
            w.io2sv().bit(enabled);
            w
        });
    }
}

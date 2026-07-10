//! RCC driver for STM32U5

use stm32u5::{Periph, stm32u5a5 as pac};

/// A list of peripherals we can enable and disable
pub enum Peripheral {
    /// GTZC on AHB1
    Gtzc,
    /// USART1 on APB2
    Usart1,
    /// SRAM3
    Sram3,
    /// GPIOA
    GpioA,
    /// GPIOB
    GpioB,
    /// GPIOC
    GpioC,
    /// GPIOD
    GpioD,
    /// GPIOE
    GpioE,
    /// GPIOF
    GpioF,
    /// GPIOG
    GpioG,
    /// GPIOH
    GpioH,
    /// GPIOI
    GpioI,
    /// GPIOJ
    GpioJ,
    /// PWR
    Power,
    // FLASH
    Flash,
}

/// A basic RCC driver
pub struct Driver<const ADDR: usize> {
    pac_object: Periph<pac::rcc::RegisterBlock, ADDR>,
}

impl<const ADDR: usize> Driver<ADDR> {
    /// Create a new RCC driver, from a PAC object
    pub fn new(pac_object: Periph<pac::rcc::RegisterBlock, ADDR>) -> Self {
        Self { pac_object }
    }

    /// Enable/disable a peripheral in the RCC
    pub fn enable(&mut self, peripheral: Peripheral, enabled: bool) {
        // write to the correct ENR register, then read it again to make sure
        // the write has flushed (which is what the STM32 C HAL does).
        match peripheral {
            Peripheral::Gtzc => {
                self.pac_object.ahb1enr().modify(|_r, w| {
                    w.gtzc1en().bit(enabled);
                    w
                });
                let _ = self.pac_object.ahb1enr().read();
            }
            Peripheral::Usart1 => {
                self.pac_object.apb2enr().modify(|_r, w| {
                    w.usart1en().bit(enabled);
                    w
                });
                let _ = self.pac_object.apb2enr().read();
            }
            Peripheral::Sram3 => {
                self.pac_object.ahb2enr1().modify(|_r, w| {
                    w.sram3en().bit(enabled);
                    w
                });
                let _ = self.pac_object.ahb2enr1().read();
            }
            Peripheral::GpioA => {
                self.pac_object.ahb2enr1().modify(|_r, w| {
                    w.gpioaen().bit(enabled);
                    w
                });
                let _ = self.pac_object.ahb2enr1().read();
            }
            Peripheral::GpioB => {
                self.pac_object.ahb2enr1().modify(|_r, w| {
                    w.gpioben().bit(enabled);
                    w
                });
                let _ = self.pac_object.ahb2enr1().read();
            }
            Peripheral::GpioC => {
                self.pac_object.ahb2enr1().modify(|_r, w| {
                    w.gpiocen().bit(enabled);
                    w
                });
                let _ = self.pac_object.ahb2enr1().read();
            }
            Peripheral::GpioD => {
                self.pac_object.ahb2enr1().modify(|_r, w| {
                    w.gpioden().bit(enabled);
                    w
                });
                let _ = self.pac_object.ahb2enr1().read();
            }
            Peripheral::GpioE => {
                self.pac_object.ahb2enr1().modify(|_r, w| {
                    w.gpioeen().bit(enabled);
                    w
                });
                let _ = self.pac_object.ahb2enr1().read();
            }
            Peripheral::GpioF => {
                self.pac_object.ahb2enr1().modify(|_r, w| {
                    w.gpiofen().bit(enabled);
                    w
                });
                let _ = self.pac_object.ahb2enr1().read();
            }
            Peripheral::GpioG => {
                self.pac_object.ahb2enr1().modify(|_r, w| {
                    w.gpiogen().bit(enabled);
                    w
                });
                let _ = self.pac_object.ahb2enr1().read();
            }
            Peripheral::GpioH => {
                self.pac_object.ahb2enr1().modify(|_r, w| {
                    w.gpiohen().bit(enabled);
                    w
                });
                let _ = self.pac_object.ahb2enr1().read();
            }
            Peripheral::GpioI => {
                self.pac_object.ahb2enr1().modify(|_r, w| {
                    w.gpioien().bit(enabled);
                    w
                });
                let _ = self.pac_object.ahb2enr1().read();
            }
            Peripheral::GpioJ => {
                self.pac_object.ahb2enr1().modify(|_r, w| {
                    w.gpiojen().bit(enabled);
                    w
                });
                let _ = self.pac_object.ahb2enr1().read();
            }
            Peripheral::Power => {
                self.pac_object.ahb3enr().modify(|_r, w| {
                    w.pwren().bit(enabled);
                    w
                });
                let _ = self.pac_object.ahb3enr().read();
            }
            Peripheral::Flash => {
                self.pac_object.ahb1enr().modify(|_r, w| {
                    w.flashen().bit(enabled);
                    w
                });
                let _ = self.pac_object.ahb1enr().read();
            }
        }
    }
}

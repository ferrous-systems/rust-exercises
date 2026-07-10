//! # nucleo-u5a5zj-bsp
//!
//! A small BSP for the NUCLEO-U5A5ZJ-Q board

#![no_std]

pub use stm32u5::stm32u5a5 as pac;
pub use stm32u5_tiny_hal::{
    self as hal,
    gpio::{Output, SecureOutput},
};

pub struct SecureBoard {
    /// USART1, connected to the USB Virtual COM Port
    pub usart1: hal::usart::Driver<0x5001_3800>,
    /// Secure Attribution Unit
    pub sau: cortex_m::peripheral::SAU,
    /// Global TrustZone Controller
    pub gztc: hal::gtzc::Driver,
    /// Nonsecure System Control Block
    pub scb_ns: cortex_m::peripheral::SCBNS,
    /// Secure System Control Block
    pub scb: cortex_m::peripheral::SCB,
    /// GPIO driver
    pub gpio: hal::gpio::SecureDriver,
    /// Power Control
    pub pwr: hal::pwr::Driver<0x5602_0800>,
    /// Green LED
    pub green_ld1: SecureLed,
    /// Blue LED
    pub blue_ld2: SecureLed,
    /// Red LED
    pub red_ld3: SecureLed,
}

impl SecureBoard {
    /// Grab the secure board support package
    ///
    /// Will panic if you've already grabbed either the [`SecureBoard`] or the [`NonSecureBoard`]
    pub fn new() -> Self {
        let p = pac::Peripherals::take().expect("Grabbed peripherals twice?!");
        let cp = cortex_m::Peripherals::take().expect("Grabbed core peripherals twice?");

        // Enable all the peripherals we need
        let mut rcc = hal::rcc::Driver::new(p.SEC_RCC);
        rcc.enable(hal::rcc::Peripheral::Usart1, true);
        rcc.enable(hal::rcc::Peripheral::Sram3, true);
        rcc.enable(hal::rcc::Peripheral::Gtzc, true);
        rcc.enable(hal::rcc::Peripheral::Power, true);
        rcc.enable(hal::rcc::Peripheral::Flash, true);

        // Let's check if they set the board up correctly

        let tzen = p.SEC_FLASH.optr().read().tzen().bit_is_set();
        if !tzen {
            panic!("Run the 'step1-option-bytes' program to set TZEN=1");
        }
        let secure_watermark2 = p.SEC_FLASH.secwm2r1().read();
        if secure_watermark2.secwm2_pstrt() != 0xFF || secure_watermark2.secwm2_pend() != 0 {
            panic!("Run the 'step2-secure-watermark' program to unprotect Flash Bank 2");
        }

        // Enable the USB Virtual COM Port UART
        let mut usart1 = hal::usart::Driver::new(p.SEC_USART1);
        usart1.configure();

        let (mut gpio, pins) = hal::gpio::SecureDriver::new(
            p.SEC_GPIOA,
            p.SEC_GPIOB,
            p.SEC_GPIOC,
            p.SEC_GPIOD,
            p.SEC_GPIOE,
            p.SEC_GPIOF,
            p.SEC_GPIOG,
            p.SEC_GPIOH,
            p.SEC_GPIOI,
            p.SEC_GPIOJ,
            &mut rcc,
        );

        let gztc = hal::gtzc::Driver::new(
            p.SEC_GTZC1_MPCBB1,
            p.SEC_GTZC1_MPCBB2,
            p.SEC_GTZC1_MPCBB3,
            p.SEC_GTZC1_MPCBB5,
        );

        let mut pwr = hal::pwr::Driver::new(p.SEC_PWR);
        pwr.vddio2_enable(true);

        let green_ld1 = SecureLed {
            inner: gpio.change_to_output(pins.port_c.pin7),
        };
        let blue_ld2 = SecureLed {
            inner: gpio.change_to_output(pins.port_b.pin7),
        };
        let red_ld3 = SecureLed {
            inner: gpio.change_to_output(pins.port_g.pin2),
        };

        Self {
            usart1,
            sau: cp.SAU,
            gztc,
            scb_ns: cp.SCBNS,
            scb: cp.SCB,
            gpio,
            pwr,
            green_ld1,
            blue_ld2,
            red_ld3,
        }
    }

    /// Set SRAM3 to be nonsecure
    ///
    /// Still allows secure mode to have read/write access to it
    pub fn set_sram3_nonsecure(&mut self) {
        self.gztc
            .map_addresses_nonsecure(
                hal::gtzc::SramBank::SRAM3,
                0x0000_0000..hal::ns_addr::SRAM3_LEN,
            )
            .expect("map_addresses_nonsecure");
    }

    /// Configure the SAU
    pub fn configure_sau(&mut self) {
        use cortex_m::peripheral::sau::{SauRegion, SauRegionAttribute};
        use hal::ns_addr;

        // These symbols come from the cortex-m-rt linker script
        unsafe extern "C" {
            static __veneer_base: u32;
            static __veneer_limit: u32;
        }

        // Nonsecure Flash
        self.sau
            .init(&[
                SauRegion {
                    base_address: ns_addr::FLASH2_START as u32,
                    limit_address: ns_addr::FLASH2_END as u32,
                    attribute: SauRegionAttribute::NonSecure,
                },
                SauRegion {
                    base_address: ns_addr::SRAM3_START as u32,
                    limit_address: ns_addr::SRAM3_END as u32,
                    attribute: SauRegionAttribute::NonSecure,
                },
                SauRegion {
                    base_address: ns_addr::PERIPH_START as u32,
                    limit_address: ns_addr::PERIPH_END as u32,
                    attribute: SauRegionAttribute::NonSecure,
                },
                SauRegion {
                    base_address: (&raw const __veneer_base) as u32,
                    limit_address: ((&raw const __veneer_limit) as u32) - 1,
                    attribute: SauRegionAttribute::NonSecureCallable,
                },
            ])
            .expect("Programming SAU");
        self.sau.enable();
    }
}

impl Default for SecureBoard {
    fn default() -> Self {
        Self::new()
    }
}

pub struct NonSecureBoard {
    /// GPIO driver
    pub gpio: hal::gpio::NonsecureDriver,
    /// Green LED
    pub green_ld1: Led,
    /// Blue LED
    pub blue_ld2: Led,
    /// Red LED
    pub red_ld3: Led,
}

impl NonSecureBoard {
    /// Grab the non-secure board support package
    ///
    /// Will panic if you've already grabbed either the [`SecureBoard`] or the [`NonSecureBoard`]
    pub fn new() -> Self {
        let p = pac::Peripherals::take().expect("Grabbed peripherals twice?!");
        let (mut gpio, pins) = hal::gpio::NonsecureDriver::new(
            p.GPIOA, p.GPIOB, p.GPIOC, p.GPIOD, p.GPIOE, p.GPIOF, p.GPIOG, p.GPIOH, p.GPIOI,
            p.GPIOJ,
        );

        let green_ld1 = Led {
            inner: gpio.change_to_output(pins.port_c.pin7),
        };
        let blue_ld2 = Led {
            inner: gpio.change_to_output(pins.port_b.pin7),
        };
        let red_ld3 = Led {
            inner: gpio.change_to_output(pins.port_g.pin2),
        };

        Self {
            gpio,
            green_ld1,
            blue_ld2,
            red_ld3,
        }
    }
}

impl Default for NonSecureBoard {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents an LED on the board
pub struct SecureLed {
    inner: SecureOutput,
}

impl SecureLed {
    /// Turn LED on
    pub fn on(&self) {
        self.inner.set_high();
    }

    /// Turn LED off
    pub fn off(&self) {
        self.inner.set_low();
    }

    /// Make this LED available the nonsecure world
    pub fn make_nonsecure(self, gpio: &mut hal::gpio::SecureDriver) {
        let secure_input = gpio.change_to_input(self.inner);
        let _input = gpio.change_to_nonsecure_input(secure_input);
    }
}

/// Represents an LED on the board
pub struct Led {
    inner: Output,
}

impl Led {
    /// Turn LED on
    pub fn on(&self) {
        self.inner.set_high();
    }

    /// Turn LED off
    pub fn off(&self) {
        self.inner.set_low();
    }
}

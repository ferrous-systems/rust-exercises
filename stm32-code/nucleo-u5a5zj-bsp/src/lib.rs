//! # nucleo-u5a5zj-bsp
//!
//! A small BSP for the NUCLEO-U5A5ZJ-Q board

#![no_std]
#![deny(missing_docs)]
#![deny(clippy::missing_safety_doc)]

use rand::SeedableRng;
pub use stm32u5::stm32u5a5 as pac;
pub use stm32u5_tiny_hal::{
    self as hal,
    gpio::{Output, SecureOutput},
};

/// Medium Speed Internal System (MSIS) Clock power-on default value
pub const MSIS_CLK_HZ: u32 = 4_000_000;

/// AHB Clock power-on default value
pub const HCLK_HZ: u32 = MSIS_CLK_HZ;

/// APB2 Peripheral Clock power-on default value
pub const APB2_PERIPH_CLK_HZ: u32 = HCLK_HZ;

/// Drivers for the NUCLEO-U5A5ZJ-Q board when running in Secure State
pub struct SecureBoard {
    /// USART1, connected to the USB Virtual COM Port
    pub usart1: hal::usart::Driver<{ hal::usart::USART1_S }>,
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
    /// Create the secure board support package, using existing resources
    pub fn new_with(mut cp: cortex_m::Peripherals, p: pac::Peripherals) -> Self {
        // trace must be enabled for cycle counter to work
        cp.DCB.enable_trace();
        // we use the cycle counter as a crude 8 MHz power-on timer
        cp.DWT.disable_cycle_counter();
        cp.DWT.set_cycle_count(0);
        cp.DWT.enable_cycle_counter();

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

        // Create a driver for the UART connected to the USB Virtual COM Port
        let usart1 = hal::usart::Driver::new(p.SEC_USART1);

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
        // We need VDDIO2 enabled for the Red LED to work
        pwr.vddio2_enable(true);

        // Set PC7, PB7 and PG2 to be outputs, because they are the LED pins
        //
        // A secure state application may choose to switch these pins over to Nonsecure State mode
        let green_ld1 = SecureLed {
            inner: gpio.change_to_output(pins.port_c.pin7),
        };
        let blue_ld2 = SecureLed {
            inner: gpio.change_to_output(pins.port_b.pin7),
        };
        let red_ld3 = SecureLed {
            inner: gpio.change_to_output(pins.port_g.pin2),
        };

        // Set PA9 and PA10 to their "USART1_TX/RX" alternate function (AF7)
        gpio.change_to_af(pins.port_a.pin9, 7);
        gpio.change_to_af(pins.port_a.pin10, 7);

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

    /// Create the secure board support package
    ///
    /// Will panic if you've already grabbed either the `cortex-m` peripherals or the PAC peripherals.
    pub fn new() -> Self {
        let cp = cortex_m::Peripherals::take().expect("Grabbed core peripherals twice?");
        let p = pac::Peripherals::take().expect("Grabbed peripherals twice?!");
        Self::new_with(cp, p)
    }

    /// Set SRAM3 to be nonsecure
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

        self.sau
            .init(&[
                // Nonsecure Flash (second bank)
                SauRegion {
                    base_address: ns_addr::FLASH2_START as u32,
                    limit_address: ns_addr::FLASH2_END as u32,
                    attribute: SauRegionAttribute::NonSecure,
                },
                // Nonsecure SRAM (SRAM3)
                SauRegion {
                    base_address: ns_addr::SRAM3_START as u32,
                    limit_address: ns_addr::SRAM3_END as u32,
                    attribute: SauRegionAttribute::NonSecure,
                },
                // All of the Nonsecure Peripherals
                SauRegion {
                    base_address: ns_addr::PERIPH_START as u32,
                    limit_address: ns_addr::PERIPH_END as u32,
                    attribute: SauRegionAttribute::NonSecure,
                },
                // the Secure Gateway stubs we export
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

/// Drivers for the NUCLEO-U5A5ZJ-Q board when running in Nonsecure State
pub struct NonSecureBoard {
    /// USART1, connected to the USB Virtual COM Port
    pub usart1: hal::usart::Driver<{ hal::usart::USART1_NS }>,
    /// GPIO driver
    pub gpio: hal::gpio::NonsecureDriver,
    /// Green LED
    pub green_ld1: Led,
    /// Blue LED
    pub blue_ld2: Led,
    /// Red LED
    pub red_ld3: Led,
    /// A small random number generator
    pub rng: rand::rngs::SmallRng,
}

impl NonSecureBoard {
    /// Create the nonsecure board support package, using existing resources
    pub fn new_with(cp: &mut cortex_m::Peripherals, p: pac::Peripherals) -> Self {
        // Enable all the peripherals we need
        let mut rcc = hal::rcc::Driver::new(p.RCC);
        rcc.enable(hal::rcc::Peripheral::Usart1, true);
        rcc.enable(hal::rcc::Peripheral::GpioA, true);
        rcc.enable(hal::rcc::Peripheral::GpioB, true);
        rcc.enable(hal::rcc::Peripheral::GpioC, true);
        rcc.enable(hal::rcc::Peripheral::GpioG, true);
        rcc.enable(hal::rcc::Peripheral::Power, true);

        // trace must be enabled for cycle counter to work
        cp.DCB.enable_trace();
        // we use the cycle counter as a crude 8 MHz power-on timer
        // but first we grab the counter as a randon number seed
        cp.DWT.enable_cycle_counter();
        let cycle_count = cortex_m::peripheral::DWT::cycle_count();
        // now lets reset it so the timestamps make more sense
        cp.DWT.disable_cycle_counter();
        cp.DWT.set_cycle_count(0);
        cp.DWT.enable_cycle_counter();

        let rng = rand::rngs::SmallRng::seed_from_u64(cycle_count as u64);

        // Create a driver for the UART connected to the USB Virtual COM Port
        let usart1 = hal::usart::Driver::new(p.USART1);

        let (mut gpio, pins) = hal::gpio::NonsecureDriver::new(
            p.GPIOA, p.GPIOB, p.GPIOC, p.GPIOD, p.GPIOE, p.GPIOF, p.GPIOG, p.GPIOH, p.GPIOI,
            p.GPIOJ,
        );

        let mut pwr = hal::pwr::Driver::new(p.PWR);
        // We need VDDIO2 enabled for the Red LED to work
        pwr.vddio2_enable(true);

        // Set PC7, PB7 and PG2 to be outputs, because they are the LED pins
        //
        // (+) This will have no effect unless either:
        //
        // * TZEN=0, or
        // * Secure Mode switched these GPIO pins to Nonsecure mode.
        //
        // However if neither of those is true then this is harmless (it gets
        // ignored), so we do it anyway.

        let green_ld1 = Led {
            inner: gpio.change_to_output(pins.port_c.pin7),
        };
        let blue_ld2 = Led {
            inner: gpio.change_to_output(pins.port_b.pin7),
        };
        let red_ld3 = Led {
            inner: gpio.change_to_output(pins.port_g.pin2),
        };

        // Set PA9 and PA10 to their "USART1_TX/RX" alternate function (AF7)
        //
        // See note (+) above about the effect of GPIO pin changes in Nonsecure
        // State
        gpio.change_to_af(pins.port_a.pin9, 7);
        gpio.change_to_af(pins.port_a.pin10, 7);

        Self {
            usart1,
            gpio,
            green_ld1,
            blue_ld2,
            red_ld3,
            rng,
        }
    }

    /// Create the nonsecure board support package
    ///
    /// Will panic if you've already grabbed either the `cortex-m` peripherals or the PAC peripherals.
    pub fn new() -> NonSecureBoard {
        let mut cp = cortex_m::Peripherals::take().expect("Grabbed core peripherals twice?");
        let p = pac::Peripherals::take().expect("Grabbed peripherals twice?!");
        NonSecureBoard::new_with(&mut cp, p)
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

/// Get the system timestamp, in microseconds
///
/// Gets a timestamp that you can use with `defmt::timestamp!`
///
/// ```rust,ignore
/// defmt::timestamp!("{=u32:tus}", bsp::timestamp());
/// ```
pub fn timestamp() -> u32 {
    // We run at 4 MHz because we never bother to reprogram the clock.
    // Therefore cycles / 4 = microseconds
    cortex_m::peripheral::DWT::cycle_count() / 4
}

// These re-exports allow us to use this crate as an RTIC device crate

pub use pac::NVIC_PRIO_BITS;
pub use pac::Peripherals;
pub use pac::interrupt;

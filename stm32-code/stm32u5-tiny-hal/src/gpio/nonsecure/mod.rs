//! Nonsecure State GPIO functionality

use crate::gpio::{PinInner, PinKind, Pull, private};
use stm32u5::stm32u5a5 as pac;

/// Types for Nonsecure pins in different modes
pub mod pin_mode;

/// Nonsecure GPIO driver
///
/// Controls all ports on the system, as one
pub struct Driver {
    inner: super::Driver,
}

/// All the NonSecure GPIO ports
pub struct Ports {
    pub port_a: PinsForPort,
    pub port_b: PinsForPort,
    pub port_c: PinsForPort,
    pub port_d: PinsForPort,
    pub port_e: PinsForPort,
    pub port_f: PinsForPort,
    pub port_g: PinsForPort,
    pub port_h: PinsForPort,
    pub port_i: PinsForPort,
    pub port_j: PinsForPort,
}

/// All the pins in a port, for NonSecure State
pub struct PinsForPort {
    pub pin0: pin_mode::Analog,
    pub pin1: pin_mode::Analog,
    pub pin2: pin_mode::Analog,
    pub pin3: pin_mode::Analog,
    pub pin4: pin_mode::Analog,
    pub pin5: pin_mode::Analog,
    pub pin6: pin_mode::Analog,
    pub pin7: pin_mode::Analog,
    pub pin8: pin_mode::Analog,
    pub pin9: pin_mode::Analog,
    pub pin10: pin_mode::Analog,
    pub pin11: pin_mode::Analog,
    pub pin12: pin_mode::Analog,
    pub pin13: pin_mode::Analog,
    pub pin14: pin_mode::Analog,
    pub pin15: pin_mode::Analog,
}

impl Driver {
    /// Create a new GPIO driver object
    pub fn new(
        _gpioa: pac::GPIOA,
        _gpiob: pac::GPIOB,
        _gpioc: pac::GPIOC,
        _gpiod: pac::GPIOD,
        _gpioe: pac::GPIOE,
        _gpiof: pac::GPIOF,
        _gpiog: pac::GPIOG,
        _gpioh: pac::GPIOH,
        _gpioi: pac::GPIOI,
        _gpioj: pac::GPIOJ,
        rcc: &mut crate::rcc::Driver<0x4602_0C00>,
    ) -> (Self, Ports) {
        rcc.enable(crate::rcc::Peripheral::GpioA, true);
        rcc.enable(crate::rcc::Peripheral::GpioB, true);
        rcc.enable(crate::rcc::Peripheral::GpioC, true);
        rcc.enable(crate::rcc::Peripheral::GpioD, true);
        rcc.enable(crate::rcc::Peripheral::GpioE, true);
        rcc.enable(crate::rcc::Peripheral::GpioF, true);
        rcc.enable(crate::rcc::Peripheral::GpioG, true);
        rcc.enable(crate::rcc::Peripheral::GpioH, true);
        rcc.enable(crate::rcc::Peripheral::GpioI, true);
        rcc.enable(crate::rcc::Peripheral::GpioJ, true);

        let driver = Driver {
            inner: super::Driver {
                _phantom: core::marker::PhantomData,
            },
        };

        let pins = Ports {
            port_a: Self::make_pins(0x00),
            port_b: Self::make_pins(0x10),
            port_c: Self::make_pins(0x20),
            port_d: Self::make_pins(0x30),
            port_e: Self::make_pins(0x40),
            port_f: Self::make_pins(0x50),
            port_g: Self::make_pins(0x60),
            port_h: Self::make_pins(0x70),
            port_i: Self::make_pins(0x80),
            port_j: Self::make_pins(0x90),
        };

        (driver, pins)
    }

    const fn make_pins(start: u8) -> PinsForPort {
        PinsForPort {
            pin0: pin_mode::Analog(PinInner(start)),
            pin1: pin_mode::Analog(PinInner(start + 1)),
            pin2: pin_mode::Analog(PinInner(start + 2)),
            pin3: pin_mode::Analog(PinInner(start + 3)),
            pin4: pin_mode::Analog(PinInner(start + 4)),
            pin5: pin_mode::Analog(PinInner(start + 5)),
            pin6: pin_mode::Analog(PinInner(start + 6)),
            pin7: pin_mode::Analog(PinInner(start + 7)),
            pin8: pin_mode::Analog(PinInner(start + 8)),
            pin9: pin_mode::Analog(PinInner(start + 9)),
            pin10: pin_mode::Analog(PinInner(start + 10)),
            pin11: pin_mode::Analog(PinInner(start + 11)),
            pin12: pin_mode::Analog(PinInner(start + 12)),
            pin13: pin_mode::Analog(PinInner(start + 13)),
            pin14: pin_mode::Analog(PinInner(start + 14)),
            pin15: pin_mode::Analog(PinInner(start + 15)),
        }
    }

    /// Change mode of a pin to Output
    pub fn change_to_output(&mut self, pin: impl PinKind<false>) -> pin_mode::Output {
        let pin_inner = pin.degrade(private::Token());
        self.inner.change_to_output(&pin_inner);
        pin_mode::Output(pin_inner)
    }

    /// Change mode of a pin to Input
    pub fn change_to_input(&mut self, pin: impl PinKind<false>, pull: Pull) -> pin_mode::Input {
        let pin_inner = pin.degrade(private::Token());
        self.inner.change_to_input(&pin_inner, pull);
        pin_mode::Input(pin_inner)
    }

    /// Change mode of a pin to Analog
    pub fn change_to_analog(&mut self, pin: impl PinKind<false>) -> pin_mode::Analog {
        let pin_inner = pin.degrade(private::Token());
        self.inner.change_to_analog(&pin_inner);
        pin_mode::Analog(pin_inner)
    }

    /// Change mode of a pin to Alternate Function (AF)
    pub fn change_to_af(&mut self, pin: impl PinKind<false>, af_mode: u8) -> pin_mode::Af {
        let pin_inner = pin.degrade(private::Token());
        self.inner.change_to_af(&pin_inner, af_mode);
        pin_mode::Af(pin_inner)
    }
}

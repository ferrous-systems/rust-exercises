//! GPIO driver for STM32U5

use stm32u5::stm32u5a5 as pac;

/// Represents ownership of a GPIO pin
#[derive(Debug)]
struct PinInner<const SECURE: bool>(u8);

impl<const SECURE: bool> PinInner<SECURE> {
    pub fn get_port_pin(&self) -> (Port, u8) {
        let upper = self.0 >> 4;
        let lower = self.0 & 0x0F;
        let port = match upper {
            0 => Port::A,
            1 => Port::B,
            2 => Port::C,
            3 => Port::D,
            4 => Port::E,
            5 => Port::F,
            6 => Port::G,
            7 => Port::H,
            8 => Port::I,
            9 => Port::J,
            _ => unreachable!(),
        };
        (port, lower)
    }

    pub fn get_port_mask(&self) -> (Port, u16) {
        let (port, pin) = self.get_port_pin();
        (port, 1 << pin)
    }

    /// Is this pin high?
    fn get(&self) -> bool {
        let (port, mask) = self.get_port_mask();
        let base_ptr = port.base(SECURE);
        let idr = unsafe { base_ptr.byte_offset(0x10).read_volatile() };
        (idr & (mask as u32)) != 0
    }

    /// Set this pin high or low
    fn set(&self, high: bool) {
        let (port, mask) = self.get_port_mask();
        let mask = if high {
            mask as u32
        } else {
            (mask as u32) << 16
        };
        let base_ptr = port.base(SECURE);
        unsafe { base_ptr.byte_offset(0x18).write_volatile(mask) };
    }
}

/// A Pin in input mode
#[derive(Debug)]
pub struct Input(PinInner<false>);

impl Input {
    pub fn is_high(&self) -> bool {
        self.0.get()
    }
}

/// A Pin in output mode
#[derive(Debug)]
pub struct Output(PinInner<false>);

impl Output {
    /// Set pin high
    pub fn set_high(&self) {
        self.set(true);
    }

    /// Set pin low
    pub fn set_low(&self) {
        self.set(false);
    }

    /// Set pin high/low
    pub fn set(&self, high: bool) {
        self.0.set(high);
    }
}

/// A Secure Pin in input mode
#[derive(Debug)]
pub struct SecureInput(PinInner<true>);

impl SecureInput {
    pub fn is_high(&self) -> bool {
        self.0.get()
    }
}

/// A Secure Pin in output mode
#[derive(Debug)]
pub struct SecureOutput(PinInner<true>);

impl SecureOutput {
    /// Set pin high
    pub fn set_high(&self) {
        self.set(true);
    }

    /// Set pin low
    pub fn set_low(&self) {
        self.set(false);
    }

    /// Set pin high/low
    pub fn set(&self, high: bool) {
        self.0.set(high);
    }
}

/// Lists all the GPIO ports on the system
///
/// Each port has up to 16 pins
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Port {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
}

impl Port {
    fn base(&self, secure: bool) -> *mut u32 {
        match self {
            Port::A => {
                if secure {
                    pac::SEC_GPIOA::PTR as *mut u32
                } else {
                    pac::GPIOA::PTR as *mut u32
                }
            }
            Port::B => {
                if secure {
                    pac::SEC_GPIOB::PTR as *mut u32
                } else {
                    pac::GPIOB::PTR as *mut u32
                }
            }
            Port::C => {
                if secure {
                    pac::SEC_GPIOC::PTR as *mut u32
                } else {
                    pac::GPIOC::PTR as *mut u32
                }
            }
            Port::D => {
                if secure {
                    pac::SEC_GPIOD::PTR as *mut u32
                } else {
                    pac::GPIOD::PTR as *mut u32
                }
            }
            Port::E => {
                if secure {
                    pac::SEC_GPIOE::PTR as *mut u32
                } else {
                    pac::GPIOE::PTR as *mut u32
                }
            }
            Port::F => {
                if secure {
                    pac::SEC_GPIOF::PTR as *mut u32
                } else {
                    pac::GPIOF::PTR as *mut u32
                }
            }
            Port::G => {
                if secure {
                    pac::SEC_GPIOG::PTR as *mut u32
                } else {
                    pac::GPIOG::PTR as *mut u32
                }
            }
            Port::H => {
                if secure {
                    pac::SEC_GPIOH::PTR as *mut u32
                } else {
                    pac::GPIOH::PTR as *mut u32
                }
            }
            Port::I => {
                if secure {
                    pac::SEC_GPIOI::PTR as *mut u32
                } else {
                    pac::GPIOI::PTR as *mut u32
                }
            }
            Port::J => {
                if secure {
                    pac::SEC_GPIOJ::PTR as *mut u32
                } else {
                    pac::GPIOJ::PTR as *mut u32
                }
            }
        }
    }
}

/// Secure GPIO driver
///
/// Controls all ports on the system, as one
pub struct SecureDriver {
    _phantom: core::marker::PhantomData<*mut u32>,
}

pub struct SecurePins {
    pub port_a: SecurePinsForPort,
    pub port_b: SecurePinsForPort,
    pub port_c: SecurePinsForPort,
    pub port_d: SecurePinsForPort,
    pub port_e: SecurePinsForPort,
    pub port_f: SecurePinsForPort,
    pub port_g: SecurePinsForPort,
    pub port_h: SecurePinsForPort,
    pub port_i: SecurePinsForPort,
    pub port_j: SecurePinsForPort,
}

pub struct SecurePinsForPort {
    pub pin0: SecureInput,
    pub pin1: SecureInput,
    pub pin2: SecureInput,
    pub pin3: SecureInput,
    pub pin4: SecureInput,
    pub pin5: SecureInput,
    pub pin6: SecureInput,
    pub pin7: SecureInput,
    pub pin8: SecureInput,
    pub pin9: SecureInput,
    pub pin10: SecureInput,
    pub pin11: SecureInput,
    pub pin12: SecureInput,
    pub pin13: SecureInput,
    pub pin14: SecureInput,
    pub pin15: SecureInput,
}

impl SecureDriver {
    /// Create a new GPIO driver object
    pub fn new(
        _gpioa: pac::SEC_GPIOA,
        _gpiob: pac::SEC_GPIOB,
        _gpioc: pac::SEC_GPIOC,
        _gpiod: pac::SEC_GPIOD,
        _gpioe: pac::SEC_GPIOE,
        _gpiof: pac::SEC_GPIOF,
        _gpiog: pac::SEC_GPIOG,
        _gpioh: pac::SEC_GPIOH,
        _gpioi: pac::SEC_GPIOI,
        _gpioj: pac::SEC_GPIOJ,
        rcc: &mut crate::rcc::Driver<1442974720>,
    ) -> (Self, SecurePins) {
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

        let driver = SecureDriver {
            _phantom: core::marker::PhantomData,
        };

        let pins = SecurePins {
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

    const fn make_pins(start: u8) -> SecurePinsForPort {
        SecurePinsForPort {
            pin0: SecureInput(PinInner(start + 0)),
            pin1: SecureInput(PinInner(start + 1)),
            pin2: SecureInput(PinInner(start + 2)),
            pin3: SecureInput(PinInner(start + 3)),
            pin4: SecureInput(PinInner(start + 4)),
            pin5: SecureInput(PinInner(start + 5)),
            pin6: SecureInput(PinInner(start + 6)),
            pin7: SecureInput(PinInner(start + 7)),
            pin8: SecureInput(PinInner(start + 8)),
            pin9: SecureInput(PinInner(start + 9)),
            pin10: SecureInput(PinInner(start + 10)),
            pin11: SecureInput(PinInner(start + 11)),
            pin12: SecureInput(PinInner(start + 12)),
            pin13: SecureInput(PinInner(start + 13)),
            pin14: SecureInput(PinInner(start + 14)),
            pin15: SecureInput(PinInner(start + 15)),
        }
    }

    /// Change pin mode
    pub fn change_to_output(&mut self, input: SecureInput) -> SecureOutput {
        let (port, pin) = input.0.get_port_pin();
        self.change_mode(port, pin, 0b01);
        SecureOutput(input.0)
    }

    /// Change pin mode
    pub fn change_to_input(&mut self, output: SecureOutput) -> SecureInput {
        let (port, pin) = output.0.get_port_pin();
        self.change_mode(port, pin, 0b00);
        SecureInput(output.0)
    }

    /// Change pin security
    pub fn change_to_nonsecure_input(&mut self, secure_input: SecureInput) -> Input {
        let (port, mask) = secure_input.0.get_port_mask();
        // mask contains a 1 bit for our pin
        // we want all 1s but a 0 bit for our pin
        let mask = !(mask as u32);
        unsafe {
            let seccfg_ptr = port.base(true).byte_offset(0x30);
            let existing = seccfg_ptr.read_volatile();
            let new = existing & mask;
            seccfg_ptr.write_volatile(new);
        }

        Input(PinInner(secure_input.0.0))
    }

    fn change_mode(&mut self, port: Port, pin: u8, bits: u8) {
        let mask = 0b11 << (pin * 2);
        let new_value = (bits as u32) << (pin * 2);
        unsafe {
            let mode_ptr = port.base(true).byte_offset(0x00);
            let existing = mode_ptr.read_volatile();
            let new = (existing & !mask) | new_value;
            mode_ptr.write_volatile(new);
        }
    }
}

/// Nonsecure GPIO driver
///
/// Controls all ports on the system, as one
pub struct NonsecureDriver {
    _phantom: core::marker::PhantomData<*mut u32>,
}

pub struct NonsecurePins {
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

pub struct PinsForPort {
    pub pin0: Input,
    pub pin1: Input,
    pub pin2: Input,
    pub pin3: Input,
    pub pin4: Input,
    pub pin5: Input,
    pub pin6: Input,
    pub pin7: Input,
    pub pin8: Input,
    pub pin9: Input,
    pub pin10: Input,
    pub pin11: Input,
    pub pin12: Input,
    pub pin13: Input,
    pub pin14: Input,
    pub pin15: Input,
}

impl NonsecureDriver {
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
    ) -> (Self, NonsecurePins) {
        let driver = NonsecureDriver {
            _phantom: core::marker::PhantomData,
        };

        let pins = NonsecurePins {
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
            pin0: Input(PinInner(start + 0)),
            pin1: Input(PinInner(start + 1)),
            pin2: Input(PinInner(start + 2)),
            pin3: Input(PinInner(start + 3)),
            pin4: Input(PinInner(start + 4)),
            pin5: Input(PinInner(start + 5)),
            pin6: Input(PinInner(start + 6)),
            pin7: Input(PinInner(start + 7)),
            pin8: Input(PinInner(start + 8)),
            pin9: Input(PinInner(start + 9)),
            pin10: Input(PinInner(start + 10)),
            pin11: Input(PinInner(start + 11)),
            pin12: Input(PinInner(start + 12)),
            pin13: Input(PinInner(start + 13)),
            pin14: Input(PinInner(start + 14)),
            pin15: Input(PinInner(start + 15)),
        }
    }

    /// Change pin mode
    pub fn change_to_output(&mut self, input: Input) -> Output {
        let (port, pin) = input.0.get_port_pin();
        self.change_mode(port, pin, 0b01);
        Output(input.0)
    }

    /// Change pin mode
    pub fn change_to_input(&mut self, output: Output) -> Input {
        let (port, pin) = output.0.get_port_pin();
        self.change_mode(port, pin, 0b00);
        Input(output.0)
    }

    fn change_mode(&mut self, port: Port, pin: u8, bits: u8) {
        let mask = 0b11 << (pin * 2);
        let new_value = (bits as u32) << (pin * 2);
        unsafe {
            let mode_ptr = port.base(false).byte_offset(0x00);
            let existing = mode_ptr.read_volatile();
            let new = (existing & !mask) | new_value;
            mode_ptr.write_volatile(new);
        }
    }
}

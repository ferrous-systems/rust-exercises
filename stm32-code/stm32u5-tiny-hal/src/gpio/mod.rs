//! GPIO driver for STM32U5

use stm32u5::stm32u5a5 as pac;

pub mod nonsecure;
pub mod secure;

const GPIO_MODER_OFFSET: isize = 0x00;
const GPIO_PUPDR_OFFSET: isize = 0x0C;
const GPIO_IDR_OFFSET: isize = 0x10;
const GPIO_BSRR_OFFSET: isize = 0x18;
const GPIO_AFRL_OFFSET: isize = 0x20;
const GPIO_AFRH_OFFSET: isize = 0x24;
const GPIO_SECCFGR_OFFSET: isize = 0x30;

/// General GPIO pin functionality
///
/// This type carries the functionality that is shared across the different
/// kinds of Pin ([Input], [Output], [SecureInput], etc).
#[doc(hidden)]
#[derive(Debug)]
pub struct PinInner<const SECURE: bool>(pub(crate) u8);

impl<const SECURE: bool> PinInner<SECURE> {
    /// Get the port (`A..=J`) and pin number (`0..=15`)
    pub(crate) fn get_port_pin(&self) -> (Port, u8) {
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

    /// Get the port (`A..=J`) and pin mask (`1 << pin_number`)
    pub(crate) fn get_port_mask(&self) -> (Port, u16) {
        let (port, pin) = self.get_port_pin();
        (port, 1 << pin)
    }

    /// Read the Input Data Register (IDR) bit for this pin
    pub(crate) fn read_idr(&self) -> bool {
        let (port, mask) = self.get_port_mask();
        let base_ptr = port.base(SECURE);
        let idr = unsafe { base_ptr.byte_offset(GPIO_IDR_OFFSET).read_volatile() };
        (idr & (mask as u32)) != 0
    }

    /// Write the Output Data Register (ODR) bit for this pin
    ///
    /// Uses the Bit Set/Reset Register to do this atomically
    pub(crate) fn write_odr(&self, high: bool) {
        let (port, mask) = self.get_port_mask();
        let mask = if high {
            mask as u32
        } else {
            (mask as u32) << 16
        };
        let base_ptr = port.base(SECURE);
        unsafe { base_ptr.byte_offset(GPIO_BSRR_OFFSET).write_volatile(mask) };
    }
}

/// All our kinds of GPIO Pin implement this
///
/// It allows the `change_to_input` (etc) APIs to take any kind of pin instead
/// of one specific kind of pin.
///
/// The const generic `SECURE` records whether the pin is assigned to Secure
/// State (true) or Nonsecure State (false).
pub trait PinKind<const SECURE: bool> {
    fn degrade(self, token: private::Token) -> PinInner<SECURE>;
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

/// The Pull on an Input
pub enum Pull {
    /// Pin floating
    None = 0b00,
    /// Pin pulled up to Vcc
    Up = 0b01,
    /// Pin pulled down to ground
    Down = 0b10,
}

/// Mode bits for a pin
enum Mode {
    Input = 0b00,
    Output = 0b01,
    AltFunc = 0b10,
    Analog = 0b11,
}

/// Hidden implementation details
///
/// Means we can have a public trait that no-one outside this crate can use,
/// because it requires this public type that has a private name.
mod private {
    pub struct Token();
}

/// GPIO driver that works for either Secure or NonSecure mode
pub struct Driver {
    _phantom: core::marker::PhantomData<*mut u32>,
}

impl Driver {
    /// Change mode of a pin to Input
    pub fn change_to_input<const S: bool>(&mut self, pin_inner: &PinInner<S>, pull: Pull) {
        let (port, pin) = pin_inner.get_port_pin();
        self.change_mode(S, port, pin, Mode::Input, pull);
    }

    /// Change mode of a pin to Output
    pub fn change_to_output<const S: bool>(&mut self, pin_inner: &PinInner<S>) {
        let (port, pin) = pin_inner.get_port_pin();
        self.change_mode(S, port, pin, Mode::Output, Pull::None);
    }

    /// Change mode of a pin to Alternate Function (AF)
    pub fn change_to_af<const S: bool>(&mut self, pin_inner: &PinInner<S>, af_mode: u8) {
        let (port, pin) = pin_inner.get_port_pin();
        // set AF field
        let (af_ptr, shift) = if pin <= 7 {
            // use AF_LOW register
            let af_ptr = unsafe { port.base(S).byte_offset(GPIO_AFRL_OFFSET) };
            (af_ptr, pin * 4)
        } else {
            // use AF_HIGH register
            let af_ptr = unsafe { port.base(S).byte_offset(GPIO_AFRH_OFFSET) };
            (af_ptr, (pin - 8) * 4)
        };
        let mask = 0xF << shift;
        let new_value = (u32::from(af_mode & 0xF)) << shift;
        unsafe {
            let existing = af_ptr.read_volatile();
            let new = (existing & !mask) | new_value;
            af_ptr.write_volatile(new);
        }
        // go to AF mode
        self.change_mode(S, port, pin, Mode::AltFunc, Pull::None);
    }

    /// Change mode of a pin to Analog
    pub fn change_to_analog<const S: bool>(&mut self, pin_inner: &PinInner<S>) {
        let (port, pin) = pin_inner.get_port_pin();
        self.change_mode(S, port, pin, Mode::Analog, Pull::None);
    }

    /// General routine to change MODER and PUPDR for a pin on a port
    fn change_mode(&mut self, secure: bool, port: Port, pin: u8, mode: Mode, pull: Pull) {
        let mask = 0b11 << (pin * 2);
        let mode_value = (mode as u32) << (pin * 2);
        let pupd_value = (pull as u32) << (pin * 2);
        unsafe {
            let mode_ptr = port.base(secure).byte_offset(GPIO_MODER_OFFSET);
            let existing = mode_ptr.read_volatile();
            let new = (existing & !mask) | mode_value;
            mode_ptr.write_volatile(new);

            let pupd_ptr = port.base(secure).byte_offset(GPIO_PUPDR_OFFSET);
            let existing = pupd_ptr.read_volatile();
            let new = (existing & !mask) | pupd_value;
            pupd_ptr.write_volatile(new);
        }
    }
}

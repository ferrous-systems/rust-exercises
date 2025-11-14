//! Board Support Package (BSP) for the nRF52840 Dongle
//!
//! See <https://www.nordicsemi.com/Products/Development-hardware/nrf52840-dk>

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use defmt_rtt as _; // global logger

pub use bsp_shared::*;
pub use hal;
use hal::gpio;
pub use hal::pac::{interrupt, Interrupt, NVIC_PRIO_BITS, RTC0};

/// Exports PAC peripherals
pub mod peripheral {
    pub use hal::pac::{interrupt, Interrupt, POWER, USBD};
}

hal::bind_interrupts!(
    struct Irqs {
        RADIO => hal::radio::InterruptHandler<hal::peripherals::RADIO>;
    }
);

// A short-hand for the nRF52 USB types
//pub type UsbBus = hal::usbd::Usbd<hal::usbd::UsbPeripheral<'static>>;

/// Components on the board
pub struct Board {
    /// LEDs
    pub leds: Leds,
    /// Timer
    pub timer: Timer,

    /// Radio interface
    pub radio: hal::radio::ieee802154::Radio<'static>,
    /// UBBD peripheral instance from the embassy peripheral singleton.
    pub usbd: hal::Peri<'static, hal::peripherals::USBD>,
    /// UBBD peripheral register block.
    pub usbd_regs: hal::pac::usbd::Usbd,
}

/// RGB LED (LD2)
pub struct RgbLed {
    /// LD2 Red: pin P0.08
    pub red: Led,
    /// LD2 Green: pin P1.09
    pub green: Led,
    /// LD2 Blue: pin P0.12
    pub blue: Led,
}

impl RgbLed {
    /// Steal the RGB LED pins.
    ///
    /// # Safety
    ///
    /// Circumvents the ownership checks provided by the HAL/BSP.
    pub unsafe fn steal() -> Self {
        let periph = hal::Peripherals::steal();
        Self {
            red: Led {
                inner: gpio::Output::new(
                    unsafe { hal::Peripherals::steal() }.P0_08,
                    gpio::Level::High,
                    gpio::OutputDrive::Standard,
                ),
                port: gpio::Port::Port0,
                pin: 8,
            },
            green: Led {
                inner: gpio::Output::new(
                    periph.P1_09,
                    gpio::Level::High,
                    gpio::OutputDrive::Standard,
                ),
                port: gpio::Port::Port1,
                pin: 9,
            },
            blue: Led {
                inner: gpio::Output::new(
                    periph.P0_12,
                    gpio::Level::High,
                    gpio::OutputDrive::Standard,
                ),
                port: gpio::Port::Port0,
                pin: 12,
            },
        }
    }

    /// Turn off all colors
    pub fn off(&mut self) {
        self.red.off();
        self.green.off();
        self.blue.off();
    }

    /// Switch on the red color only.
    pub fn red_only(&mut self) {
        self.red.on();
        self.green.off();
        self.blue.off();
    }

    /// Switch on the green color only.
    pub fn green_only(&mut self) {
        self.green.on();
        self.red.off();
        self.blue.off();
    }

    /// Switch on the blue color only.
    pub fn blue_only(&mut self) {
        self.blue.on();
        self.red.off();
        self.green.off();
    }
}

/// All LEDs on the board
///
/// See User Manual Table 1
pub struct Leds {
    /// LD1: pin P0.06
    pub ld1_green: Led,
    /// LD2 RGB: pins P0.08 (red), P1.09 (green), P0.12 (blue)
    pub ld2_rgb: RgbLed,
}

/// A byte-based ring-buffer that you can writeln! into, and drain under
/// interrupt.
///
/// Used for buffering serial port output.
///
/// Stores 128 bytes, maximum.
pub struct Ringbuffer {
    buffer: heapless::mpmc::Queue<u8, 128>,
}

impl Ringbuffer {
    /// Construct a new Ringbuffer
    pub const fn new() -> Ringbuffer {
        Ringbuffer {
            buffer: heapless::mpmc::Queue::new(),
        }
    }

    /// Take an item from the buffer
    pub fn read(&self) -> Option<u8> {
        self.buffer.dequeue()
    }

    /// Add an item to the queue
    pub fn write(&self, value: u8) -> Result<(), u8> {
        self.buffer.enqueue(value)
    }
}

impl Default for Ringbuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Write for &Ringbuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            let _ = self.buffer.enqueue(b);
        }
        Ok(())
    }
}

/// Initializes the board
///
/// This return an `Err`or if called more than once
pub fn init() -> Board {
    defmt::debug!("Initializing the board");
    let mut config = hal::config::Config::default();
    config.hfclk_source = hal::config::HfclkSource::ExternalXtal;
    config.lfclk_source = hal::config::LfclkSource::ExternalXtal;
    let periph = hal::init(config);
    defmt::debug!("board peripherals have been initialized");

    let mut rtc = hal::rtc::Rtc::new(periph.RTC0, 0).unwrap();
    // NOTE on unmasking the NVIC interrupt: Because this crate defines the `#[interrupt] fn RTC0`
    // interrupt handler, RTIC cannot manage that interrupt (trying to do so results in a linker
    // error). Thus it is the task of this crate to mask/unmask the interrupt in a safe manner.
    //
    // Because the RTC0 interrupt handler does *not* access static variables through a critical
    // section (that disables interrupts) this `unmask` operation cannot break critical sections
    // and thus won't lead to undefined behavior (e.g. torn reads/writes)
    //
    // the preceding `enable_conuter` method consumes the `rtc` value. This is a semantic move
    // of the RTC0 peripheral from this function (which can only be called at most once) to the
    // interrupt handler (where the peripheral is accessed without any synchronization
    // mechanism)
    rtc.enable_interrupt(hal::rtc::Interrupt::Overflow, true);
    rtc.enable();

    defmt::debug!("RTC started");

    let timer = Timer::new(periph.TIMER0);

    let radio = {
        let mut radio = hal::radio::ieee802154::Radio::new(periph.RADIO, Irqs);

        // set TX power to its maximum value
        radio.set_transmission_power(8);
        defmt::debug!("Radio initialized and configured with TX power set to the maximum value");
        radio
    };

    Board {
        // NOTE LEDs turn on when the pin output level is low
        leds: Leds {
            ld1_green: Led {
                inner: gpio::Output::new(
                    periph.P0_06,
                    gpio::Level::High,
                    gpio::OutputDrive::Standard,
                ),
                port: gpio::Port::Port0,
                pin: 6,
            },
            // Safety: This function is called only once during board initialization.
            ld2_rgb: unsafe { RgbLed::steal() },
        },
        radio,
        timer,
        usbd: periph.USBD,
        usbd_regs: hal::pac::USBD,
    }
}

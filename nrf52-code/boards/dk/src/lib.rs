//! Board Support Package (BSP) for the nRF52840 Development Kit
//!
//! Based on [`embassy-nrf`](https://docs.embassy.dev/embassy-nrf/git/nrf52840/index.html) and
//! [`nrf-pac`](https://github.com/embassy-rs/nrf-pac).
//!
//! See <https://www.nordicsemi.com/Products/Development-hardware/nrf52840-dk>

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use core::{
    hint::spin_loop,
    sync::atomic::{self, AtomicU32, Ordering},
    time::Duration,
};

use cortex_m_semihosting::debug;
use embedded_hal::delay::DelayNs;
#[cfg(feature = "advanced")]
use grounded::uninit::GroundedArrayCell;
pub use hal;
pub use hal::pac::{interrupt, Interrupt, NVIC_PRIO_BITS, RTC0};
use hal::{
    gpio::{Level, Output, OutputDrive, Port},
    Peri,
};

#[cfg(any(feature = "radio", feature = "advanced"))]
use defmt_rtt as _; // global logger

#[cfg(feature = "advanced")]
mod errata;
pub mod peripheral;
#[cfg(feature = "radio")]
pub mod radio;
#[cfg(feature = "advanced")]
pub mod usbd;

/// Components on the board
pub struct Board {
    /// LEDs
    pub leds: Leds,
    /// Timer
    pub timer: Timer,

    /// Radio interface
    #[cfg(feature = "radio")]
    pub radio: crate::radio::Radio<'static>,
    /// USBD (Universal Serial Bus Device) peripheral
    #[cfg(any(feature = "advanced", feature = "usbd"))]
    pub usbd: hal::pac::usbd::Usbd,
    /// POWER (Power Supply) peripheral
    #[cfg(feature = "advanced")]
    pub power: hal::pac::power::Power,
    /// USB control endpoint 0
    #[cfg(feature = "advanced")]
    pub ep0in: usbd::Ep0In,
}

/// All LEDs on the board
pub struct Leds {
    /// LED1: pin P0.13, green LED
    pub _1: Led,
    /// LED2: pin P0.14, green LED
    pub _2: Led,
    /// LED3: pin P0.15, green LED
    pub _3: Led,
    /// LED4: pin P0.16, green LED
    pub _4: Led,
}

/// A single LED
pub struct Led {
    port: Port,
    pin: u8,
    inner: Output<'static>,
}

impl Led {
    /// Turns on the LED
    pub fn on(&mut self) {
        defmt::trace!(
            "setting P{}.{} low (LED on)",
            if self.port == Port::Port1 { '1' } else { '0' },
            self.pin
        );

        self.inner.set_low()
    }

    /// Turns off the LED
    pub fn off(&mut self) {
        defmt::trace!(
            "setting P{}.{} high (LED off)",
            if self.port == Port::Port1 { '1' } else { '0' },
            self.pin
        );

        self.inner.set_high()
    }

    /// Returns `true` if the LED is in the OFF state
    pub fn is_off(&mut self) -> bool {
        self.inner.is_set_high()
    }

    /// Returns `true` if the LED is in the ON state
    pub fn is_on(&mut self) -> bool {
        !self.is_off()
    }

    /// Toggles the state (on/off) of the LED
    pub fn toggle(&mut self) {
        if self.is_off() {
            self.on();
        } else {
            self.off()
        }
    }
}

/// A timer for creating blocking delays
pub struct Timer(hal::timer::Timer<'static>);

impl DelayNs for Timer {
    fn delay_ns(&mut self, ns: u32) {
        if ns == 0 {
            return;
        }
        self.0.stop();
        self.0.clear();
        // Write cycle count in microseconds for 1 MHz timer.
        self.0.cc(0).write(ns / 1_000);
        self.0.start();
        while !self.reset_if_finished() {
            spin_loop();
        }
    }
}

impl Timer {
    /// Create a new timer instance which can be used for blocking delays.
    pub fn new<T: hal::timer::Instance>(peri: Peri<'static, T>) -> Self {
        let timer = hal::timer::Timer::new(peri);
        timer.set_frequency(hal::timer::Frequency::F1MHz);
        timer.cc(0).short_compare_clear();
        timer.cc(0).short_compare_stop();
        Self(timer)
    }

    /// Start the timer with the given microsecond duration.
    pub fn start(&mut self, microseconds: u32) {
        self.0.stop();
        self.0.clear();
        self.0.cc(0).write(microseconds);
        self.0.start();
    }

    /// If the timer has finished, resets it and returns true.
    ///
    /// Returns false if the timer is still running.
    pub fn reset_if_finished(&mut self) -> bool {
        if !self.0.cc(0).event_compare().is_triggered() {
            // EVENTS_COMPARE has not been triggered yet
            return false;
        }

        self.0.cc(0).clear_events();

        true
    }

    /// Wait for the specified duration.
    pub fn wait(&mut self, duration: Duration) {
        defmt::trace!("blocking for {:?} ...", duration);

        // 1 cycle = 1 microsecond
        let subsec_micros = duration.subsec_micros();
        if subsec_micros != 0 {
            self.delay_us(subsec_micros);
        }

        let mut millis = duration.as_secs() * 1000;
        if millis == 0 {
            return;
        }

        while millis > u32::MAX as u64 {
            self.delay_ms(u32::MAX);
            millis -= u32::MAX as u64;
        }
        self.delay_ms(millis as u32);

        defmt::trace!("... DONE");
    }
}

#[cfg(feature = "radio")]
mod radio_retry {
    use embedded_hal::delay::DelayNs as _;

    use crate::radio::{self, Packet};

    const RETRY_COUNT: u32 = 10;
    const ADDR_LEN: usize = 6;

    fn get_id() -> [u8; ADDR_LEN] {
        let ficr = hal::pac::FICR;

        let id = ficr.deviceaddr(0).read();
        let id2 = ficr.deviceaddr(1).read();
        let id = u64::from(id) << 32 | u64::from(id2);
        defmt::trace!("Device ID: {:#08x}", id);
        let id_bytes = id.to_be_bytes();
        [
            id_bytes[0],
            id_bytes[1],
            id_bytes[2],
            id_bytes[3],
            id_bytes[4],
            id_bytes[5],
        ]
    }

    /// Send a packet, containing the device address and the given data, and
    /// wait for a response.
    ///
    /// If we get a response containing the same device address, it returns a
    /// slice of the remaining payload (i.e. not including the device address).
    ///
    /// If we don't get a response, or we get a bad response (with the wrong
    /// address in it), we try again.
    ///
    /// If we try too many times, we give up.
    pub fn send_recv<'packet>(
        packet: &'packet mut Packet,
        data_to_send: &[u8],
        radio: &mut crate::radio::Radio,
        timer: &mut crate::Timer,
        microseconds: u32,
    ) -> Result<&'packet [u8], crate::radio::Error> {
        assert!(data_to_send.len() + ADDR_LEN < usize::from(Packet::CAPACITY));

        let id_bytes = get_id();
        // Short delay before sending, so we don't get into a tight loop and steal all the bandwidth
        timer.delay_us(5000);
        for i in 0..RETRY_COUNT {
            packet.set_len(ADDR_LEN as u8 + data_to_send.len() as u8);
            let source_iter = id_bytes.iter().chain(data_to_send.iter());
            let dest_iter = packet.iter_mut();
            for (source, dest) in source_iter.zip(dest_iter) {
                *dest = *source;
            }
            defmt::debug!("TX: {=[u8]:02x}", &packet[..]);
            radio.send(packet);
            match radio.recv_timeout(packet, timer, microseconds) {
                Ok(_crc) => {
                    defmt::debug!("RX: {=[u8]:02x}", packet[..]);
                    // packet is long enough
                    if packet[0..ADDR_LEN] == id_bytes {
                        // and it has the right bytes at the start
                        defmt::debug!("OK: {=[u8]:02x}", packet[ADDR_LEN..]);
                        return Ok(&packet[ADDR_LEN..]);
                    } else {
                        defmt::warn!("RX Wrong Address try {}", i);
                        timer.delay_us(10000);
                    }
                }
                Err(radio::Error::Timeout) => {
                    defmt::warn!("RX Timeout try {}", i);
                    timer.delay_us(10000);
                }
                Err(radio::Error::Crc(_)) => {
                    defmt::warn!("RX CRC Error try {}", i);
                    timer.delay_us(10000);
                }
            }
        }
        Err(radio::Error::Timeout)
    }
}

#[cfg(feature = "radio")]
pub use radio_retry::send_recv;

/// The ways that initialisation can fail
#[derive(Debug, Copy, Clone, defmt::Format)]
pub enum Error {
    /// You tried to initialise the board twice
    DoubleInit = 1,
}

/// Initializes the board
///
/// This return an `Err`or if called more than once
pub fn init() -> Result<Board, Error> {
    // probe-rs puts us in blocking mode, so wait for blocking mode as a proxy
    // for waiting for probe-rs to connect.
    while !defmt_rtt::in_blocking_mode() {
        core::hint::spin_loop();
    }
    // NOTE: this branch runs at most once
    #[cfg(feature = "advanced")]
    static EP0IN_BUF: GroundedArrayCell<u8, 64> = GroundedArrayCell::const_init();

    let mut config = hal::config::Config::default();
    config.hfclk_source = hal::config::HfclkSource::ExternalXtal;
    config.lfclk_source = hal::config::LfclkSource::ExternalXtal;
    let periph = hal::init(config);

    // NOTE: this branch runs at most once

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

    let led1pin = Led {
        port: Port::Port0,
        pin: 13,
        inner: Output::new(periph.P0_13, Level::High, OutputDrive::Standard),
    };
    let led2pin = Led {
        port: Port::Port0,
        pin: 14,
        inner: Output::new(periph.P0_14, Level::High, OutputDrive::Standard),
    };
    let led3pin = Led {
        port: Port::Port0,
        pin: 15,
        inner: Output::new(periph.P0_15, Level::High, OutputDrive::Standard),
    };
    let led4pin = Led {
        port: Port::Port0,
        pin: 16,
        inner: Output::new(periph.P0_16, Level::High, OutputDrive::Standard),
    };

    defmt::debug!("I/O pins have been configured for digital output");

    let timer = Timer::new(periph.TIMER0);

    #[cfg(feature = "radio")]
    let radio = {
        use hal::radio::TxPower;

        let mut radio = crate::radio::Radio::new(periph.RADIO);

        // set TX power to its maximum value
        radio.set_transmission_power(TxPower::POS8_DBM);
        defmt::debug!("Radio initialized and configured with TX power set to the maximum value");
        radio
    };

    Ok(Board {
        leds: Leds {
            _1: led1pin,
            _2: led2pin,
            _3: led3pin,
            _4: led4pin,
        },
        #[cfg(feature = "radio")]
        radio,
        timer,
        #[cfg(feature = "advanced")]
        ep0in: unsafe { usbd::Ep0In::new(&EP0IN_BUF) },
        #[cfg(any(feature = "advanced", feature = "usbd"))]
        usbd: hal::pac::USBD,
        #[cfg(feature = "advanced")]
        power: hal::pac::POWER,
    })
}

// Counter of OVERFLOW events -- an OVERFLOW occurs every (1<<24) ticks
static OVERFLOWS: AtomicU32 = AtomicU32::new(0);

// NOTE this will run at the highest priority, higher priority than RTIC tasks
#[interrupt]
fn RTC0() {
    OVERFLOWS.fetch_add(1, Ordering::Release);
    let rtc = hal::pac::RTC0;
    // clear the EVENT register
    rtc.events_ovrflw().write_value(0);
}

/// Exits the application successfully when the program is executed through the
/// `probe-rs` Cargo runner
pub fn exit() -> ! {
    unsafe {
        // turn off the USB D+ pull-up before pausing the device with a breakpoint
        // this disconnects the nRF device from the USB host so the USB host won't attempt further
        // USB communication (and see an unresponsive device).
        const USBD_USBPULLUP: *mut u32 = 0x4002_7504 as *mut u32;
        USBD_USBPULLUP.write_volatile(0)
    }
    defmt::println!("`dk::exit()` called; exiting ...");
    // force any pending memory operation to complete before the instruction that follows
    atomic::compiler_fence(Ordering::SeqCst);
    loop {
        debug::exit(debug::ExitStatus::Ok(()))
    }
}

/// Exits the application with a failure when the program is executed through
/// the `probe-rs` Cargo runner
pub fn fail() -> ! {
    unsafe {
        // turn off the USB D+ pull-up before pausing the device with a breakpoint
        // this disconnects the nRF device from the USB host so the USB host won't attempt further
        // USB communication (and see an unresponsive device).
        const USBD_USBPULLUP: *mut u32 = 0x4002_7504 as *mut u32;
        USBD_USBPULLUP.write_volatile(0)
    }
    defmt::println!("`dk::fail()` called; exiting ...");
    // force any pending memory operation to complete before the instruction that follows
    atomic::compiler_fence(Ordering::SeqCst);
    loop {
        debug::exit(debug::ExitStatus::Err(()))
    }
}

/// Returns the time elapsed since the call to the `dk::init` function
///
/// The time is in 32,768 Hz units (i.e. 32768 = 1 second)
///
/// Calling this function before calling `dk::init` will return a value of `0` nanoseconds.
pub fn uptime_ticks() -> u64 {
    // here we are going to perform a 64-bit read of the number of ticks elapsed
    //
    // a 64-bit load operation cannot performed in a single instruction so the operation can be
    // preempted by the RTC0 interrupt handler (which increases the OVERFLOWS counter)
    //
    // the loop below will load both the lower and upper parts of the 64-bit value while preventing
    // the issue of mixing a low value with an "old" high value -- note that, due to interrupts, an
    // arbitrary amount of time may elapse between the `hi1` load and the `low` load

    // # Safety
    // Concurrent access to this field within the RTC is acceptable.
    let rtc_counter = hal::pac::RTC0.counter();

    loop {
        // NOTE volatile is used to order these load operations among themselves
        let hi1 = OVERFLOWS.load(Ordering::Acquire);
        let low = rtc_counter.read().counter();
        let hi2 = OVERFLOWS.load(Ordering::Relaxed);

        if hi1 == hi2 {
            break u64::from(low) | (u64::from(hi1) << 24);
        }
    }
}

/// Returns the time elapsed since the call to the `dk::init` function
///
/// The clock that is read to compute this value has a resolution of 30 microseconds.
///
/// Calling this function before calling `dk::init` will return a value of `0` nanoseconds.
pub fn uptime() -> Duration {
    // We have a time in 32,768 Hz units.
    let mut ticks = uptime_ticks();

    // turn it into 32_768_000_000 units
    ticks = ticks.wrapping_mul(1_000_000);
    // turn it into microsecond units
    ticks >>= 15;
    // turn it into nanosecond units
    ticks = ticks.wrapping_mul(1_000);

    // NB: 64-bit nanoseconds handles around 584 years.

    let secs = ticks / 1_000_000_000;
    let nanos = ticks % 1_000_000_000;

    Duration::new(secs, nanos as u32)
}

/// Returns the time elapsed since the call to the `dk::init` function, in microseconds.
///
/// The clock that is read to compute this value has a resolution of 30 microseconds.
///
/// Calling this function before calling `dk::init` will return a value of `0` nanoseconds.
pub fn uptime_us() -> u64 {
    // We have a time in 32,768 Hz units.
    let mut ticks = uptime_ticks();

    // turn it into 32_768_000_000 units
    ticks = ticks.wrapping_mul(1_000_000);
    // turn it into microsecond units
    ticks >>= 15;

    ticks
}

/// Returns the least-significant bits of the device identifier
pub fn deviceid0() -> u32 {
    hal::pac::FICR.deviceid(0).read()
}

/// Returns the most-significant bits of the device identifier
pub fn deviceid1() -> u32 {
    hal::pac::FICR.deviceid(1).read()
}

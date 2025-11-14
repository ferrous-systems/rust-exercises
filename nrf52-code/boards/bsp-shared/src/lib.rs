//! # Shared board support package components.
#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use core::{
    hint::spin_loop,
    sync::atomic::{AtomicU32, Ordering, compiler_fence},
    time::Duration,
};
use embedded_hal::delay::DelayNs;
use hal::gpio;
pub use hal::pac::{Interrupt, NVIC_PRIO_BITS, RTC0, interrupt};

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

/// A single LED
pub struct Led {
    /// Actual GPIO output pin controlling the LED.
    pub inner: gpio::Output<'static>,
    /// Port of the LED pin.
    pub port: gpio::Port,
    /// Pin index of the LED pin on the port.
    pub pin: u8,
}

impl Led {
    /// Turns on the LED
    pub fn on(&mut self) {
        defmt::trace!(
            "setting P{}.{} low (LED on)",
            if self.port == gpio::Port::Port1 {
                '1'
            } else {
                '0'
            },
            self.pin
        );

        self.inner.set_low()
    }

    /// Turns off the LED
    pub fn off(&mut self) {
        defmt::trace!(
            "setting P{}.{} high (LED off)",
            if self.port == gpio::Port::Port1 {
                '1'
            } else {
                '0'
            },
            self.pin
        );

        self.inner.set_high()
    }

    /// Set the LED to the specified state.
    pub fn set(&mut self, on: bool) {
        if on { self.on() } else { self.off() }
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

impl embedded_hal::delay::DelayNs for Timer {
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
    pub fn new<T: hal::timer::Instance>(peri: hal::Peri<'static, T>) -> Self {
        let timer = hal::timer::Timer::new(peri);
        timer.set_frequency(hal::timer::Frequency::F1MHz);
        timer.cc(0).short_compare_clear();
        timer.cc(0).short_compare_stop();
        Self(timer)
    }

    /// Start the timer with the given microsecond duration.
    pub fn start(&mut self, microseconds: u32) {
        self.0.cc(0).clear_events();
        self.0.cc(0).write(microseconds);
        self.0.task_clear();
        self.0.task_start();
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
    compiler_fence(Ordering::SeqCst);
    loop {
        cortex_m_semihosting::debug::exit(cortex_m_semihosting::debug::ExitStatus::Ok(()))
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
    compiler_fence(Ordering::SeqCst);
    loop {
        cortex_m_semihosting::debug::exit(cortex_m_semihosting::debug::ExitStatus::Err(()))
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

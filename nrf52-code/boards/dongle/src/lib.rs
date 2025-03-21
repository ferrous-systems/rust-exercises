//! Board Support Package (BSP) for the nRF52840 Dongle
//!
//! See <https://www.nordicsemi.com/Products/Development-hardware/nrf52840-dk>

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use core::{
    ops,
    sync::atomic::{self, AtomicU32, Ordering},
    time::Duration,
};

use cortex_m::peripheral::NVIC;
use cortex_m_semihosting::debug;
use embedded_hal::digital::{OutputPin, StatefulOutputPin};
use grounded::uninit::GroundedCell;

use hal::{
    clocks::{self, Clocks},
    gpio::{p0, p1, Level, Output, Pin, Port, PushPull},
    pac::USBD,
    rtc::{Rtc, RtcInterrupt},
    timer::OneShot,
};

use defmt_rtt as _; // global logger

pub use hal::pac::{interrupt, Interrupt, NVIC_PRIO_BITS, RTC0};
pub use hal::{self, ieee802154};

/// Exports PAC peripherals
pub mod peripheral {
    pub use hal::pac::{interrupt, Interrupt, POWER, USBD};
}

/// A short-hand for the nRF52 USB types
pub type UsbBus = hal::usbd::Usbd<hal::usbd::UsbPeripheral<'static>>;

struct ClockSyncWrapper<H, L, LSTAT> {
    clocks: Clocks<H, L, LSTAT>,
}

unsafe impl<H, L, LSTAT> Sync for ClockSyncWrapper<H, L, LSTAT> {}

/// Components on the board
pub struct Board {
    /// LEDs
    pub leds: Leds,
    /// Timer
    pub timer: Timer,

    /// Radio interface
    pub radio: ieee802154::Radio<'static>,
    /// USBD (Universal Serial Bus Device) peripheral
    pub usbd: USBD,
    /// Clocks
    pub clocks: &'static Clocks<
        clocks::ExternalOscillator,
        clocks::ExternalOscillator,
        clocks::LfOscStarted,
    >,
}

/// All LEDs on the board
///
/// See User Manual Table 1
pub struct Leds {
    /// LD1: pin P0.06
    pub ld1: Led,
    /// LD2 Red: pin P0.08
    pub ld2_red: Led,
    /// LD2 Green: pin P1.09
    pub ld2_green: Led,
    /// LD2 Blue: pin P0.12
    pub ld2_blue: Led,
}

/// A single LED
pub struct Led {
    inner: Pin<Output<PushPull>>,
}

impl Led {
    /// Turns on the LED
    pub fn on(&mut self) {
        defmt::trace!(
            "setting P{}.{} low (LED on)",
            if self.inner.port() == Port::Port1 {
                '1'
            } else {
                '0'
            },
            self.inner.pin()
        );

        // NOTE this operations returns a `Result` but never returns the `Err` variant
        let _ = self.inner.set_low();
    }

    /// Turns off the LED
    pub fn off(&mut self) {
        defmt::trace!(
            "setting P{}.{} high (LED off)",
            if self.inner.port() == Port::Port1 {
                '1'
            } else {
                '0'
            },
            self.inner.pin()
        );

        // NOTE this operations returns a `Result` but never returns the `Err` variant
        let _ = self.inner.set_high();
    }

    /// Returns `true` if the LED is in the OFF state
    pub fn is_off(&mut self) -> bool {
        self.inner.is_set_high() == Ok(true)
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
pub struct Timer {
    inner: hal::Timer<hal::pac::TIMER0, OneShot>,
}

impl Timer {
    /// Blocks program execution for at least the specified `duration`
    pub fn wait(&mut self, duration: Duration) {
        defmt::trace!("blocking for {:?} ...", duration);

        // 1 cycle = 1 microsecond
        let subsec_micros = duration.subsec_micros();
        if subsec_micros != 0 {
            self.inner.delay(subsec_micros);
        }

        const MICROS_IN_ONE_SEC: u32 = 1_000_000;
        // maximum number of seconds that fit in a single `delay` call without overflowing the `u32`
        // argument
        const MAX_SECS: u32 = u32::MAX / MICROS_IN_ONE_SEC;
        let mut secs = duration.as_secs();
        while secs != 0 {
            let cycles = if secs > MAX_SECS as u64 {
                secs -= MAX_SECS as u64;
                MAX_SECS * MICROS_IN_ONE_SEC
            } else {
                let cycles = secs as u32 * MICROS_IN_ONE_SEC;
                secs = 0;
                cycles
            };

            self.inner.delay(cycles)
        }

        defmt::trace!("... DONE");
    }

    /// Start a timer running for `timeout_us` microseconds
    pub fn start(&mut self, timeout_us: u32) {
        self.inner.start(timeout_us)
    }
}

impl ops::Deref for Timer {
    type Target = hal::Timer<hal::pac::TIMER0, OneShot>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl ops::DerefMut for Timer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// A byte-based ring-buffer that you can writeln! into, and drain under
/// interrupt.
///
/// Used for buffering serial port output.
///
/// Stores 128 bytes, maximum.
pub struct Ringbuffer {
    buffer: heapless::mpmc::MpMcQueue<u8, 128>,
}

impl Ringbuffer {
    /// Construct a new Ringbuffer
    pub const fn new() -> Ringbuffer {
        Ringbuffer {
            buffer: heapless::mpmc::MpMcQueue::new(),
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

impl core::fmt::Write for &Ringbuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            let _ = self.buffer.enqueue(b);
        }
        Ok(())
    }
}

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
    let Some(periph) = hal::pac::Peripherals::take() else {
        return Err(Error::DoubleInit);
    };
    // NOTE: this branch runs at most once
    // We need the wrapper to make this type Sync, as it contains raw pointers
    static CLOCKS: GroundedCell<
        ClockSyncWrapper<
            clocks::ExternalOscillator,
            clocks::ExternalOscillator,
            clocks::LfOscStarted,
        >,
    > = GroundedCell::uninit();
    defmt::debug!("Initializing the board");

    let clocks = Clocks::new(periph.CLOCK);
    let clocks = clocks.enable_ext_hfosc();
    let clocks = clocks.set_lfclk_src_external(clocks::LfOscConfiguration::NoExternalNoBypass);
    let clocks = clocks.start_lfclk();
    let clocks = clocks.enable_ext_hfosc();
    // extend lifetime to `'static`
    let clocks = unsafe {
        let clocks_ptr = CLOCKS.get();
        clocks_ptr.write(ClockSyncWrapper { clocks });
        // Now it's initialised, we can take a static reference to the clocks
        // object it contains.
        let clock_wrapper: &'static ClockSyncWrapper<_, _, _> = &*clocks_ptr;
        &clock_wrapper.clocks
    };
    defmt::debug!("Clocks configured");

    let mut rtc = Rtc::new(periph.RTC0, 0).unwrap();
    rtc.enable_interrupt(RtcInterrupt::Overflow, None);
    rtc.enable_counter();
    // NOTE(unsafe) because this crate defines the `#[interrupt] fn RTC0` interrupt handler,
    // RTIC cannot manage that interrupt (trying to do so results in a linker error). Thus it
    // is the task of this crate to mask/unmask the interrupt in a safe manner.
    //
    // Because the RTC0 interrupt handler does *not* access static variables through a critical
    // section (that disables interrupts) this `unmask` operation cannot break critical sections
    // and thus won't lead to undefined behavior (e.g. torn reads/writes)
    //
    // the preceding `enable_conuter` method consumes the `rtc` value. This is a semantic move
    // of the RTC0 peripheral from this function (which can only be called at most once) to the
    // interrupt handler (where the peripheral is accessed without any synchronization
    // mechanism)
    unsafe { NVIC::unmask(hal::pac::Interrupt::RTC0) };

    defmt::debug!("RTC started");

    let p0_pins = p0::Parts::new(periph.P0);
    let p1_pins = p1::Parts::new(periph.P1);

    defmt::debug!("I/O pins have been configured for digital output");

    let timer = hal::Timer::new(periph.TIMER0);

    let radio = {
        let mut radio = ieee802154::Radio::init(periph.RADIO, clocks);

        // set TX power to its maximum value
        radio.set_txpower(ieee802154::TxPower::Pos8dBm);
        defmt::debug!("Radio initialized and configured with TX power set to the maximum value");
        radio
    };

    Ok(Board {
        // NOTE LEDs turn on when the pin output level is low
        leds: Leds {
            ld1: Led {
                inner: p0_pins.p0_06.degrade().into_push_pull_output(Level::High),
            },
            ld2_red: Led {
                inner: p0_pins.p0_08.degrade().into_push_pull_output(Level::High),
            },
            ld2_green: Led {
                inner: p1_pins.p1_09.degrade().into_push_pull_output(Level::High),
            },
            ld2_blue: Led {
                inner: p0_pins.p0_12.degrade().into_push_pull_output(Level::High),
            },
        },
        radio,
        timer: Timer { inner: timer },
        usbd: periph.USBD,
        clocks,
    })
}

// Counter of OVERFLOW events -- an OVERFLOW occurs every (1<<24) ticks
static OVERFLOWS: AtomicU32 = AtomicU32::new(0);

// NOTE this will run at the highest priority, higher priority than RTIC tasks
#[interrupt]
fn RTC0() {
    OVERFLOWS.fetch_add(1, Ordering::Release);
    // # Safety
    // Concurrent access to this field within the RTC is acceptable.
    unsafe {
        let rtc = hal::pac::Peripherals::steal().RTC0;
        // clear the EVENT register
        rtc.events_ovrflw.reset();
    }
}

/// Exits the application when the program is executed through the `probe-rs` Cargo runner
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
    let rtc_counter = unsafe { &hal::pac::Peripherals::steal().RTC0.counter };

    loop {
        // NOTE volatile is used to order these load operations among themselves
        let hi1 = OVERFLOWS.load(Ordering::Acquire);
        let low = rtc_counter.read().counter().bits();
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
    // NOTE(unsafe) read-only registers, and no other use of the block
    let ficr = unsafe { &*hal::pac::FICR::ptr() };
    ficr.deviceid[0].read().deviceid().bits()
}

/// Returns the most-significant bits of the device identifier
pub fn deviceid1() -> u32 {
    // NOTE(unsafe) read-only registers, and no other use of the block
    let ficr = unsafe { &*hal::pac::FICR::ptr() };
    ficr.deviceid[1].read().deviceid().bits()
}

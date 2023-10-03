//! Hardware Abstraction Layer (HAL) for the nRF52840 Development Kit

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use core::{
    ops,
    sync::atomic::{self, AtomicU32, Ordering},
    time::Duration,
};

use cortex_m::{asm, peripheral::NVIC};
use embedded_hal::digital::v2::{OutputPin as _, StatefulOutputPin};
#[cfg(feature = "beginner")]
pub use hal::ieee802154;
pub use hal::pac::{interrupt, Interrupt, NVIC_PRIO_BITS, RTC0};
use hal::{
    clocks::{self, Clocks},
    gpio::{p0, Level, Output, Pin, Port, PushPull},
    rtc::{Rtc, RtcInterrupt},
    timer::OneShot,
};

use defmt;
#[cfg(any(feature = "beginner", feature = "advanced"))]
use defmt_rtt as _; // global logger

#[cfg(feature = "advanced")]
use crate::{
    peripheral::{POWER, USBD},
    usbd::Ep0In,
};

#[cfg(feature = "advanced")]
mod errata;
pub mod peripheral;
#[cfg(feature = "advanced")]
pub mod usbd;

/// Components on the board
pub struct Board {
    /// LEDs
    pub leds: Leds,
    /// Timer
    pub timer: Timer,

    /// Radio interface
    #[cfg(feature = "beginner")]
    pub radio: ieee802154::Radio<'static>,
    /// USBD (Universal Serial Bus Device) peripheral
    #[cfg(feature = "advanced")]
    pub usbd: USBD,
    /// POWER (Power Supply) peripheral
    #[cfg(feature = "advanced")]
    pub power: POWER,
    /// USB control endpoint 0
    #[cfg(feature = "advanced")]
    pub ep0in: Ep0In,
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
    pub fn is_off(&self) -> bool {
        self.inner.is_set_high() == Ok(true)
    }

    /// Returns `true` if the LED is in the ON state
    pub fn is_on(&self) -> bool {
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
        const NANOS_IN_ONE_MICRO: u32 = 1_000;
        let subsec_micros = duration.subsec_nanos() / NANOS_IN_ONE_MICRO;
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

/// Initializes the board
///
/// This return an `Err`or if called more than once
pub fn init() -> Result<Board, ()> {
    if let Some(periph) = hal::pac::Peripherals::take() {
        // NOTE(static mut) this branch runs at most once
        #[cfg(feature = "advanced")]
        static mut EP0IN_BUF: [u8; 64] = [0; 64];
        #[cfg(feature = "beginner")]
        static mut CLOCKS: Option<
            Clocks<clocks::ExternalOscillator, clocks::ExternalOscillator, clocks::LfOscStarted>,
        > = None;

        defmt::debug!("Initializing the board");

        let clocks = Clocks::new(periph.CLOCK);
        let clocks = clocks.enable_ext_hfosc();
        let clocks = clocks.set_lfclk_src_external(clocks::LfOscConfiguration::NoExternalNoBypass);
        let clocks = clocks.start_lfclk();
        let _clocks = clocks.enable_ext_hfosc();
        // extend lifetime to `'static`
        #[cfg(feature = "beginner")]
        let clocks = unsafe { CLOCKS.get_or_insert(_clocks) };

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
        unsafe { NVIC::unmask(Interrupt::RTC0) };

        defmt::debug!("RTC started");

        let pins = p0::Parts::new(periph.P0);

        // NOTE LEDs turn on when the pin output level is low
        let _1 = pins.p0_13.degrade().into_push_pull_output(Level::High);
        let _2 = pins.p0_14.degrade().into_push_pull_output(Level::High);
        let _3 = pins.p0_15.degrade().into_push_pull_output(Level::High);
        let _4 = pins.p0_16.degrade().into_push_pull_output(Level::High);

        defmt::debug!("I/O pins have been configured for digital output");

        let timer = hal::Timer::new(periph.TIMER0);

        #[cfg(feature = "beginner")]
        let radio = {
            let mut radio = ieee802154::Radio::init(periph.RADIO, clocks);

            // set TX power to its maximum value
            radio.set_txpower(ieee802154::TxPower::Pos8dBm);
            defmt::debug!(
                "Radio initialized and configured with TX power set to the maximum value"
            );
            radio
        };

        Ok(Board {
            leds: Leds {
                _1: Led { inner: _1 },
                _2: Led { inner: _2 },
                _3: Led { inner: _3 },
                _4: Led { inner: _4 },
            },
            #[cfg(feature = "beginner")]
            radio,
            timer: Timer { inner: timer },
            #[cfg(feature = "advanced")]
            usbd: periph.USBD,
            #[cfg(feature = "advanced")]
            power: periph.POWER,
            #[cfg(feature = "advanced")]
            ep0in: unsafe { Ep0In::new(&mut EP0IN_BUF) },
        })
    } else {
        Err(())
    }
}

// Counter of OVERFLOW events -- an OVERFLOW occurs every (1<<24) ticks
static OVERFLOWS: AtomicU32 = AtomicU32::new(0);

// NOTE this will run at the highest priority, higher priority than RTIC tasks
#[interrupt]
fn RTC0() {
    let curr = OVERFLOWS.load(Ordering::Relaxed);
    OVERFLOWS.store(curr + 1, Ordering::Relaxed);

    // clear the EVENT register
    unsafe { core::mem::transmute::<_, RTC0>(()).events_ovrflw.reset() }
}

/// Exits the application when the program is executed through the `probe-run` Cargo runner
pub fn exit() -> ! {
    unsafe {
        // turn off the USB D+ pull-up before pausing the device with a breakpoint
        // this disconnects the nRF device from the USB host so the USB host won't attempt further
        // USB communication (and see an unresponsive device). probe-run will also reset the nRF's
        // USBD peripheral when it sees the device in a halted state which has the same effect as
        // this line but that can take a while and the USB host may issue a power cycle of the USB
        // port / hub / root in the meantime, which can bring down the probe and break probe-run
        const USBD_USBPULLUP: *mut u32 = 0x4002_7504 as *mut u32;
        USBD_USBPULLUP.write_volatile(0)
    }
    defmt::println!("`dk::exit()` called; exiting ...");
    // force any pending memory operation to complete before the BKPT instruction that follows
    atomic::compiler_fence(Ordering::SeqCst);
    loop {
        asm::bkpt()
    }
}

/// Returns the time elapsed since the call to the `dk::init` function
///
/// The clock that is read to compute this value has a resolution of 30 microseconds.
///
/// Calling this function before calling `dk::init` will return a value of `0` nanoseconds.
pub fn uptime() -> Duration {
    // here we are going to perform a 64-bit read of the number of ticks elapsed
    //
    // a 64-bit load operation cannot performed in a single instruction so the operation can be
    // preempted by the RTC0 interrupt handler (which increases the OVERFLOWS counter)
    //
    // the loop below will load both the lower and upper parts of the 64-bit value while preventing
    // the issue of mixing a low value with an "old" high value -- note that, due to interrupts, an
    // arbitrary amount of time may elapse between the `hi1` load and the `low` load
    let overflows = &OVERFLOWS as *const AtomicU32 as *const u32;
    let ticks = loop {
        unsafe {
            // NOTE volatile is used to order these load operations among themselves
            let hi1 = overflows.read_volatile();
            let low = core::mem::transmute::<_, RTC0>(())
                .counter
                .read()
                .counter()
                .bits();
            let hi2 = overflows.read_volatile();

            if hi1 == hi2 {
                break u64::from(low) | (u64::from(hi1) << 24);
            }
        }
    };

    // 2**15 ticks = 1 second
    let freq = 1 << 15;
    let secs = ticks / freq;
    // subsec ticks
    let ticks = (ticks % freq) as u32;
    // one tick is equal to `1e9 / 32768` nanos
    // the fraction can be reduced to `1953125 / 64`
    // which can be further decomposed as `78125 * (5 / 4) * (5 / 4) * (1 / 4)`.
    // Doing the operation this way we can stick to 32-bit arithmetic without overflowing the value
    // at any stage
    let nanos =
        (((ticks % 32768).wrapping_mul(78125) >> 2).wrapping_mul(5) >> 2).wrapping_mul(5) >> 2;
    Duration::new(secs, nanos as u32)
}

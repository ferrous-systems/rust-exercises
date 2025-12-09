//! Board Support Package (BSP) for the nRF52840 Development Kit
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
use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin};
#[cfg(feature = "advanced")]
use grounded::uninit::GroundedArrayCell;
#[cfg(any(feature = "radio", feature = "usbd"))]
use grounded::uninit::GroundedCell;
#[cfg(feature = "radio")]
pub use hal::ieee802154;
pub use hal::pac::{interrupt, Interrupt, NVIC_PRIO_BITS, RTC0};
use hal::{
    clocks::{self, Clocks},
    gpio::{p0, Input, Level, Output, Pin, Port, PullUp, PushPull},
    rtc::{Rtc, RtcInterrupt},
    timer::OneShot,
};

#[cfg(any(feature = "radio", feature = "advanced"))]
use defmt_rtt as _; // global logger

#[cfg(feature = "advanced")]
mod errata;
pub mod peripheral;
#[cfg(feature = "advanced")]
pub mod usbd;

#[cfg(any(feature = "radio", feature = "usbd"))]
struct ClockSyncWrapper<H, L, LSTAT> {
    clocks: Clocks<H, L, LSTAT>,
}

#[cfg(any(feature = "radio", feature = "usbd"))]
unsafe impl<H, L, LSTAT> Sync for ClockSyncWrapper<H, L, LSTAT> {}

/// Our USB Device
#[cfg(feature = "usbd")]
pub type UsbDevice = hal::usbd::Usbd<hal::usbd::UsbPeripheral<'static>>;

/// Components on the board
pub struct Board {
    /// LEDs
    pub leds: Leds,
    /// Buttons
    pub buttons: Buttons,
    /// Timer
    pub timer: Timer,

    /// Radio interface
    #[cfg(feature = "radio")]
    pub radio: ieee802154::Radio<'static>,
    /// USBD (Universal Serial Bus Device) peripheral
    #[cfg(any(feature = "advanced", feature = "usbd"))]
    pub usbd: hal::pac::USBD,
    /// POWER (Power Supply) peripheral
    #[cfg(feature = "advanced")]
    pub power: hal::pac::POWER,
    /// USB control endpoint 0
    #[cfg(feature = "advanced")]
    pub ep0in: usbd::Ep0In,
    /// Represents our current clock setup
    #[cfg(any(feature = "radio", feature = "usbd"))]
    pub clocks: &'static Clocks<
        clocks::ExternalOscillator,
        clocks::ExternalOscillator,
        clocks::LfOscStarted,
    >,
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

/// All Buttons on the board
pub struct Buttons {
    /// Button1: pin P0.11
    pub _1: Button,
    /// Button2: pin P0.12
    pub _2: Button,
    /// Button3: pin P0.24
    pub _3: Button,
    /// Button4: pin P0.25
    pub _4: Button,
}

/// A single Button
pub struct Button {
    inner: Pin<Input<PullUp>>,
}

impl Button {
    /// Is the button pressed
    pub fn is_pressed(&mut self) -> bool {
        self.inner.is_low() == Ok(true)
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

#[cfg(feature = "radio")]
mod radio_retry {
    use super::ieee802154::Packet;

    const RETRY_COUNT: u32 = 10;
    const ADDR_LEN: usize = 6;

    fn get_id() -> [u8; ADDR_LEN] {
        let ficr = unsafe { &*hal::pac::FICR::ptr() };
        let id = ficr.deviceaddr[0].read().bits();
        let id2 = ficr.deviceaddr[1].read().bits();
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
    pub fn send_recv<'packet, I>(
        packet: &'packet mut Packet,
        data_to_send: &[u8],
        radio: &mut hal::ieee802154::Radio,
        timer: &mut hal::timer::Timer<I>,
        microseconds: u32,
    ) -> Result<&'packet [u8], hal::ieee802154::Error>
    where
        I: hal::timer::Instance,
    {
        assert!(data_to_send.len() + ADDR_LEN < usize::from(Packet::CAPACITY));

        let id_bytes = get_id();
        // Short delay before sending, so we don't get into a tight loop and steal all the bandwidth
        timer.delay(5000);
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
                        timer.delay(10000);
                    }
                }
                Err(hal::ieee802154::Error::Timeout) => {
                    defmt::warn!("RX Timeout try {}", i);
                    timer.delay(10000);
                }
                Err(hal::ieee802154::Error::Crc(_)) => {
                    defmt::warn!("RX CRC Error try {}", i);
                    timer.delay(10000);
                }
            }
        }
        Err(hal::ieee802154::Error::Timeout)
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
    let Some(periph) = hal::pac::Peripherals::take() else {
        return Err(Error::DoubleInit);
    };
    // NOTE: this branch runs at most once
    #[cfg(feature = "advanced")]
    static EP0IN_BUF: GroundedArrayCell<u8, 64> = GroundedArrayCell::const_init();
    #[cfg(any(feature = "radio", feature = "usbd"))]
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
    let _clocks = clocks.enable_ext_hfosc();
    // extend lifetime to `'static`
    #[cfg(any(feature = "radio", feature = "usbd"))]
    let clocks = unsafe {
        let clocks_ptr = CLOCKS.get();
        clocks_ptr.write(ClockSyncWrapper { clocks: _clocks });
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
    unsafe { NVIC::unmask(Interrupt::RTC0) };

    defmt::debug!("RTC started");

    let pins = p0::Parts::new(periph.P0);

    // NOTE LEDs turn on when the pin output level is low
    let led1pin = pins.p0_13.degrade().into_push_pull_output(Level::High);
    let led2pin = pins.p0_14.degrade().into_push_pull_output(Level::High);
    let led3pin = pins.p0_15.degrade().into_push_pull_output(Level::High);
    let led4pin = pins.p0_16.degrade().into_push_pull_output(Level::High);

    // NOTE pin goes low when button is pressed
    let button1pin = pins.p0_11.degrade().into_pullup_input();
    let button2pin = pins.p0_12.degrade().into_pullup_input();
    let button3pin = pins.p0_24.degrade().into_pullup_input();
    let button4pin = pins.p0_25.degrade().into_pullup_input();

    defmt::debug!("I/O pins have been configured for digital output");

    let timer = hal::Timer::new(periph.TIMER0);

    #[cfg(feature = "radio")]
    let radio = {
        let mut radio = ieee802154::Radio::init(periph.RADIO, clocks);

        // set TX power to its maximum value
        radio.set_txpower(ieee802154::TxPower::Pos8dBm);
        defmt::debug!("Radio initialized and configured with TX power set to the maximum value");
        radio
    };

    #[cfg(feature = "usbd")]
    {
        defmt::debug!("Enabling SOF interrupts...");
        periph.USBD.inten.modify(|_r, w| {
            w.sof().set_bit();
            w
        });
    }

    Ok(Board {
        leds: Leds {
            _1: Led { inner: led1pin },
            _2: Led { inner: led2pin },
            _3: Led { inner: led3pin },
            _4: Led { inner: led4pin },
        },
        buttons: Buttons {
            _1: Button { inner: button1pin },
            _2: Button { inner: button2pin },
            _3: Button { inner: button3pin },
            _4: Button { inner: button4pin },
        },
        #[cfg(feature = "radio")]
        radio,
        timer: Timer { inner: timer },
        #[cfg(any(feature = "advanced", feature = "usbd"))]
        usbd: periph.USBD,
        #[cfg(feature = "advanced")]
        power: periph.POWER,
        #[cfg(feature = "advanced")]
        ep0in: unsafe { usbd::Ep0In::new(&EP0IN_BUF) },
        #[cfg(any(feature = "radio", feature = "usbd"))]
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

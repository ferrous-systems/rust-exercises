//! # IEEE 802.15.4 radio driver
//!
//! This driver is a synchronous/blocking one written for a radio training.
//! A lot of its implementation is based on the
//! [nrf52840 HAL](https://github.com/nrf-rs/nrf-hal)
//!
//! [MIT license from the project](https://github.com/nrf-rs/nrf-hal/blob/master/LICENSE-MIT):
//!
//! Copyright (c) 2018 Anthony James Munns
//!
//! Permission is hereby granted, free of charge, to any
//! person obtaining a copy of this software and associated
//! documentation files (the "Software"), to deal in the
//! Software without restriction, including without
//! limitation the rights to use, copy, modify, merge,
//! publish, distribute, sublicense, and/or sell copies of
//! the Software, and to permit persons to whom the Software
//! is furnished to do so, subject to the following
//! conditions:
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
//! ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
//! TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
//! PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
//! SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
//! CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
//! OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
//! IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
//! DEALINGS IN THE SOFTWARE.

use core::sync::atomic::{compiler_fence, Ordering};

use crate::Peri;
use hal::pac::radio::regs::Shorts;
use hal::pac::radio::vals::{self, Crcstatus, State as RadioState};
pub use hal::radio::TxPower;

/// Default (IEEE compliant) Start of Frame Delimiter
pub const DEFAULT_SFD: u8 = 0xA7;

/// Error
#[derive(Copy, Clone, Debug, PartialEq, defmt::Format)]
pub enum Error {
    /// Incorrect CRC
    Crc(u16),
    /// Timeout
    Timeout,
}

/// IEEE 802.15.4 channels
///
/// NOTE these are NOT the same as WiFi 2.4 GHz channels
#[derive(Debug, Copy, Clone, PartialEq, Eq, defmt::Format)]
pub enum Channel {
    /// 2_405 MHz
    _11 = 11,
    /// 2_410 MHz
    _12 = 12,
    /// 2_415 MHz
    _13 = 13,
    /// 2_420 MHz
    _14 = 14,
    /// 2_425 MHz
    _15 = 15,
    /// 2_430 MHz
    _16 = 16,
    /// 2_435 MHz
    _17 = 17,
    /// 2_440 MHz
    _18 = 18,
    /// 2_445 MHz
    _19 = 19,
    /// 2_450 MHz
    _20 = 20,
    /// 2_455 MHz
    _21 = 21,
    /// 2_460 MHz
    _22 = 22,
    /// 2_465 MHz
    _23 = 23,
    /// 2_470 MHz
    _24 = 24,
    /// 2_475 MHz
    _25 = 25,
    /// 2_480 MHz
    _26 = 26,
}

impl Channel {
    /// Frequency offset for the given channel.
    pub const fn frequency_offset(&self) -> u32 {
        (*self as u32 - 10) * 5
    }
}

/// Driver state
///
/// After, or at the start of, any method call the RADIO will be in one of these states
// This is a subset of the STATE_A enum
#[derive(Copy, Clone, PartialEq, Eq, defmt::Format)]
enum State {
    Disabled,
    RxIdle,
    TxIdle,
}

#[derive(Debug, defmt::Format)]
enum Event {
    PhyEnd,
}

/// Non-blocking receive
pub struct Recv<'a, 'c> {
    radio: &'a mut Radio<'c>,
}

impl<'a, 'c> Recv<'a, 'c> {
    fn new(radio: &'a mut Radio<'c>) -> Self {
        Self { radio }
    }

    /// Check if receive is done
    ///
    /// This methods returns the `Ok` variant if the CRC included the
    /// packet was successfully validated by the hardware. It returns
    /// `Err(nb::Error::WouldBlock)` if a packet hasn't been received
    /// yet, and `Err(nb::Error::Other)` if the CRC check failed.
    pub fn is_done(&self) -> nb::Result<u16, u16> {
        let regs = self.radio.regs();
        if regs.events_end().read() == 1 {
            regs.events_end().write_value(0);

            dma_end_fence();

            let crc = regs.rxcrc().read().rxcrc() as u16;

            if regs.crcstatus().read().crcstatus() == Crcstatus::CRCOK {
                Ok(crc)
            } else {
                Err(nb::Error::Other(crc))
            }
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<'a, 'c> Drop for Recv<'a, 'c> {
    fn drop(&mut self) {
        self.radio.cancel_recv();
    }
}

// TODO expose the other variants in `pac::CCAMODE_A`
/// Clear Channel Assessment method
pub enum Cca {
    /// Carrier sense
    CarrierSense,
    /// Energy Detection / Energy Above Threshold
    EnergyDetection {
        /// Energy measurements above this value mean that the channel is assumed to be busy.
        /// Note the measurement range is 0..0xFF - where 0 means that the received power was
        /// less than 10 dB above the selected receiver sensitivity. This value is not given in dBm,
        /// but can be converted. See the nrf52840 Product Specification Section 6.20.12.4
        /// for details.
        ed_threshold: u8,
    },
}

/// IEEE 802.15.4 radio driver.
pub struct Radio<'d> {
    _p: Peri<'d, hal::peripherals::RADIO>,
    needs_enable: bool,
}

impl<'d> Radio<'d> {
    /// Create a new IEEE 802.15.4 radio driver.
    pub fn new(radio: Peri<'d, hal::peripherals::RADIO>) -> Self {
        let r = hal::pac::RADIO;

        // Disable and enable to reset peripheral
        r.power().write(|w| w.set_power(false));
        r.power().write(|w| w.set_power(true));

        // Enable 802.15.4 mode
        r.mode()
            .write(|w| w.set_mode(vals::Mode::IEEE802154_250KBIT));
        // Configure CRC skip address
        r.crccnf().write(|w| {
            w.set_len(vals::Len::TWO);
            w.set_skipaddr(vals::Skipaddr::IEEE802154);
        });
        // Configure CRC polynomial and init
        r.crcpoly().write(|w| w.set_crcpoly(0x0001_1021));
        r.crcinit().write(|w| w.set_crcinit(0));
        r.pcnf0().write(|w| {
            // 8-bit on air length
            w.set_lflen(8);
            // Zero bytes S0 field length
            w.set_s0len(false);
            // Zero bytes S1 field length
            w.set_s1len(0);
            // Do not include S1 field in RAM if S1 length > 0
            w.set_s1incl(vals::S1incl::AUTOMATIC);
            // Zero code Indicator length
            w.set_cilen(0);
            // 32-bit zero preamble
            w.set_plen(vals::Plen::_32BIT_ZERO);
            // Include CRC in length
            w.set_crcinc(vals::Crcinc::INCLUDE);
        });
        r.pcnf1().write(|w| {
            // Maximum packet length
            w.set_maxlen(Packet::MAX_PSDU_LEN);
            // Zero static length
            w.set_statlen(0);
            // Zero base address length
            w.set_balen(0);
            // Little-endian
            w.set_endian(vals::Endian::LITTLE);
            // Disable packet whitening
            w.set_whiteen(false);
        });

        // Enable NVIC interrupt
        //T::Interrupt::unpend();
        //unsafe { T::Interrupt::enable() };

        let mut radio = Self {
            _p: radio,
            needs_enable: false,
        };

        radio.set_sfd(DEFAULT_SFD);
        radio.set_transmission_power_raw(0);
        radio.set_channel_raw(11);
        radio.set_cca(Cca::CarrierSense);

        radio
    }

    /// Raw register block access.
    #[inline]
    pub fn regs(&self) -> hal::pac::radio::Radio {
        hal::pac::RADIO
    }

    /// Set the channel.
    pub fn set_channel(&mut self, channel: Channel) {
        self.set_channel_raw(channel as u8);
    }

    /// Changes the radio channel using a raw channel ID.
    pub fn set_channel_raw(&mut self, channel: u8) {
        let r = self.regs();
        if !(11..=26).contains(&channel) {
            panic!("Bad 802.15.4 channel");
        }
        let frequency_offset = (channel - 10) * 5;
        self.needs_enable = true;
        r.frequency().write(|w| {
            w.set_frequency(frequency_offset);
            w.set_map(vals::Map::DEFAULT);
        });
    }

    /// Changes the Clear Channel Assessment method
    pub fn set_cca(&mut self, cca: Cca) {
        let r = self.regs();
        self.needs_enable = true;
        match cca {
            Cca::CarrierSense => r
                .ccactrl()
                .write(|w| w.set_ccamode(hal::pac::radio::vals::Ccamode::CARRIER_MODE)),
            Cca::EnergyDetection { ed_threshold } => {
                // "[ED] is enabled by first configuring the field CCAMODE=EdMode in CCACTRL
                // and writing the CCAEDTHRES field to a chosen value."
                r.ccactrl().write(|w| {
                    w.set_ccamode(hal::pac::radio::vals::Ccamode::ED_MODE);
                    w.set_ccaedthres(ed_threshold);
                });
            }
        }
    }

    /// Changes the Start of Frame Delimiter (SFD)
    pub fn set_sfd(&mut self, sfd: u8) {
        let r = self.regs();
        r.sfd().write(|w| w.set_sfd(sfd));
    }

    /// Clear interrupts
    pub fn clear_all_interrupts(&mut self) {
        let r = self.regs();
        r.intenclr().write(|w| w.0 = 0xffff_ffff);
    }

    /// Changes the radio transmission power
    pub fn set_transmission_power_raw(&mut self, power: i8) {
        self.needs_enable = true;

        let tx_power: TxPower = match power {
            8 => TxPower::POS8_DBM,
            7 => TxPower::POS7_DBM,
            6 => TxPower::POS6_DBM,
            5 => TxPower::POS5_DBM,
            4 => TxPower::POS4_DBM,
            3 => TxPower::POS3_DBM,
            2 => TxPower::POS2_DBM,
            0 => TxPower::_0_DBM,
            -4 => TxPower::NEG4_DBM,
            -8 => TxPower::NEG8_DBM,
            -12 => TxPower::NEG12_DBM,
            -16 => TxPower::NEG16_DBM,
            -20 => TxPower::NEG20_DBM,
            -30 => TxPower::NEG30_DBM,
            -40 => TxPower::NEG40_DBM,
            _ => panic!("Invalid transmission power value"),
        };

        self.set_transmission_power(tx_power);
    }

    /// Changes the radio transmission power
    pub fn set_transmission_power(&mut self, power: TxPower) {
        let r = self.regs();
        self.needs_enable = true;
        r.txpower().write(|w| w.set_txpower(power));
    }

    /// Waits until the radio state matches the given `state`
    pub fn wait_for_radio_state(&self, state: RadioState) {
        while self.regs().state().read().state() != state {}
    }

    /// Get the current radio state
    fn state(&self) -> State {
        match self.regs().state().read().state() {
            // final states
            RadioState::DISABLED => State::Disabled,
            RadioState::TX_IDLE => State::TxIdle,
            RadioState::RX_IDLE => State::RxIdle,

            // transitory states
            RadioState::TX_DISABLE => {
                self.wait_for_state_a(RadioState::DISABLED);
                State::Disabled
            }

            _ => unreachable!(),
        }
    }

    fn set_buffer(&mut self, buffer: &[u8]) {
        self.regs().packetptr().write_value(buffer.as_ptr() as u32);
    }

    /// Sample the received signal power (i.e. the presence of possibly interfering signals)
    /// within the bandwidth of the currently used channel for `sample_cycles` iterations.
    /// Note that one iteration has a sample time of 128μs, and that each iteration produces the
    /// average RSSI value measured during this sample time.
    ///
    /// Returns the *maximum* measurement recorded during sampling as reported by the hardware (not in dBm!).
    /// The result can be used to find a suitable ED threshold for Energy Detection-based CCA mechanisms.
    ///
    /// For details, see Section 6.20.12.3 Energy detection (ED) of the PS.
    /// RSSI samples are averaged over a measurement time of 8 symbol periods (128 μs).
    pub fn energy_detection_scan(&mut self, sample_cycles: u32) -> u8 {
        let regs = self.regs();
        // Increase the time spent listening
        regs.edcnt().write(|w| w.set_edcnt(sample_cycles));

        // ensure that the shortcut between READY event and START task is disabled before putting
        // the radio into recv mode
        regs.shorts()
            .write_value(hal::pac::radio::regs::Shorts::default());
        self.put_in_rx_mode();

        // clear related events
        regs.events_edend().write_value(0);

        // start energy detection sampling
        regs.tasks_edstart().write_value(1);

        loop {
            if regs.events_edend().read() == 1 {
                // sampling period is over; collect value
                regs.events_edend().write_value(0);

                // note that since we have increased EDCNT, the EDSAMPLE register contains the
                // maximum recorded value, not the average
                let read_lvl = regs.edsample().read().edlvl();
                return read_lvl;
            }
        }
    }

    /// Receives one radio packet and copies its contents into the given `packet` buffer
    ///
    /// This methods returns the `Ok` variant if the CRC included the packet was successfully
    /// validated by the hardware; otherwise it returns the `Err` variant. In either case, `packet`
    /// will be updated with the received packet's data
    pub fn recv(&mut self, packet: &mut Packet) -> Result<u16, u16> {
        // Start non-blocking receive
        self.recv_non_blocking(packet, |recv| {
            // Block untill receive is done
            nb::block!(recv.is_done())
        })
    }

    /// Receives one radio packet and copies its contents into the given `packet` buffer
    ///
    /// This method is non-blocking
    pub fn recv_non_blocking<'a, R>(
        &'a mut self,
        packet: &'a mut Packet,
        f: impl FnOnce(&Recv<'a, 'd>) -> R,
    ) -> R {
        // Start the read
        // NOTE(unsafe)
        // The packet must live until the transfer is done. Receive is handled inside
        // a closure to ensure this
        unsafe {
            self.start_recv(packet);
        }

        let recv = Recv::new(self);
        f(&recv)
    }

    /// Listens for a packet for no longer than the specified amount of microseconds
    /// and copies its contents into the given `packet` buffer
    ///
    /// If no packet is received within the specified time then the `Timeout` error is returned
    ///
    /// If a packet is received within the time span then the packet CRC is checked. If the CRC is
    /// incorrect then the `Crc` error is returned; otherwise the `Ok` variant is returned.
    /// Note that `packet` will contain the packet in any case, even if the CRC check failed.
    ///
    /// Note that the time it takes to switch the radio to RX mode is included in the timeout count.
    /// This transition may take up to a hundred of microseconds; see the section 6.20.15.8 in the
    /// Product Specification for more details about timing
    pub fn recv_timeout(
        &mut self,
        packet: &mut Packet,
        timer: &mut super::Timer,
        microseconds: u32,
    ) -> Result<u16, Error> {
        // Start the timeout timer
        timer.start(microseconds);

        // Start non-blocking receive
        self.recv_non_blocking(packet, |recv| {
            // Check if either receive is done or timeout occured
            loop {
                match recv.is_done() {
                    Ok(crc) => {
                        break Ok(crc);
                    }
                    Err(nb::Error::Other(crc)) => {
                        break Err(Error::Crc(crc));
                    }
                    Err(nb::Error::WouldBlock) => {
                        // do nothing
                    }
                }

                if timer.reset_if_finished() {
                    // Break loop in case of timeout. Receive is
                    // cancelled when `recv` is dropped.
                    break Err(Error::Timeout);
                }
            }
        })
    }

    unsafe fn start_recv(&mut self, packet: &mut Packet) {
        let regs = self.regs();
        // NOTE we do NOT check the address of `packet` because the mutable reference ensures it's
        // allocated in RAM

        // clear related events
        regs.events_phyend().write_value(0);
        regs.events_end().write_value(0);

        self.put_in_rx_mode();

        // NOTE(unsafe) DMA transfer has not yet started
        // set up RX buffer
        self.set_buffer(packet.buffer.as_mut());

        // start transfer
        dma_start_fence();
        regs.tasks_start().write_value(1);
    }

    fn cancel_recv(&mut self) {
        let regs = self.regs();
        regs.tasks_stop().write_value(1);
        self.wait_for_state_a(RadioState::RX_IDLE);
        // DMA transfer may have been in progress so synchronize with its memory operations
        dma_end_fence();
    }

    /// Tries to send the given `packet`
    ///
    /// This method performs Clear Channel Assessment (CCA) first and sends the `packet` only if the
    /// channel is observed to be *clear* (no transmission is currently ongoing), otherwise no
    /// packet is transmitted and the `Err` variant is returned
    ///
    /// NOTE this method will *not* modify the `packet` argument. The mutable reference is used to
    /// ensure the `packet` buffer is allocated in RAM, which is required by the RADIO peripheral
    // NOTE we do NOT check the address of `packet` because the mutable reference ensures it's
    // allocated in RAM
    #[allow(clippy::result_unit_err)]
    pub fn try_send(&mut self, packet: &mut Packet) -> Result<(), ()> {
        let regs = self.regs();
        // enable radio to perform cca
        self.put_in_rx_mode();

        // clear related events
        regs.events_phyend().write_value(0);
        regs.events_end().write_value(0);

        // NOTE(unsafe) DMA transfer has not yet started
        regs.packetptr().write_value(packet.buffer.as_ptr() as u32);

        // configure radio to immediately start transmission if the channel is idle
        regs.shorts().modify(|w| {
            w.set_ccaidle_txen(true);
            w.set_end_disable(true);
        });

        // the DMA transfer will start at some point after the following write operation so
        // we place the compiler fence here
        dma_start_fence();
        // start CCA. In case the channel is clear, the data at packetptr will be sent automatically
        regs.tasks_ccastart().write_value(1);

        loop {
            if regs.events_phyend().read() == 1 {
                // transmission completed
                dma_end_fence();
                regs.events_phyend().write_value(0);
                regs.shorts().write_value(Shorts::default());
                return Ok(());
            }

            if regs.events_ccabusy().read() == 1 {
                // channel is busy
                regs.events_ccabusy().write_value(0);
                regs.shorts().write_value(Shorts::default());
                return Err(());
            }
        }
    }

    /// Sends the given `packet`
    ///
    /// This is utility method that *consecutively* calls the `try_send` method until it succeeds.
    /// Note that this approach is *not* IEEE spec compliant -- there must be delay between failed
    /// CCA attempts to be spec compliant
    ///
    /// NOTE this method will *not* modify the `packet` argument. The mutable reference is used to
    /// ensure the `packet` buffer is allocated in RAM, which is required by the RADIO peripheral
    // NOTE we do NOT check the address of `packet` because the mutable reference ensures it's
    // allocated in RAM
    pub fn send(&mut self, packet: &mut Packet) {
        let regs = self.regs();

        // enable radio to perform cca
        self.put_in_rx_mode();

        // clear related events
        regs.events_phyend().write_value(0);
        regs.events_end().write_value(0);

        // immediately start transmission if the channel is idle
        regs.shorts().modify(|w| {
            w.set_ccaidle_txen(true);
            w.set_txready_start(true);
            w.set_end_disable(true);
        });

        // the DMA transfer will start at some point after the following write operation so
        // we place the compiler fence here
        dma_start_fence();
        // NOTE(unsafe) DMA transfer has not yet started
        regs.packetptr().write_value(packet.buffer.as_ptr() as u32);

        'cca: loop {
            // start CCA (+ sending if channel is clear)
            regs.tasks_ccastart().write_value(1);

            loop {
                if regs.events_phyend().read() == 1 {
                    dma_end_fence();
                    // transmission is complete
                    regs.events_phyend().write_value(0);
                    break 'cca;
                }

                if regs.events_ccabusy().read() == 1 {
                    // channel is busy; try another CCA
                    regs.events_ccabusy().write_value(0);
                    continue 'cca;
                }
            }
        }

        regs.shorts().write_value(Shorts::default());
    }

    /// Sends the specified `packet` without first performing CCA
    ///
    /// Acknowledgment packets must be sent using this method
    ///
    /// NOTE this method will *not* modify the `packet` argument. The mutable reference is used to
    /// ensure the `packet` buffer is allocated in RAM, which is required by the RADIO peripheral
    // NOTE we do NOT check the address of `packet` because the mutable reference ensures it's
    // allocated in RAM
    pub fn send_no_cca(&mut self, packet: &mut Packet) {
        let regs = self.regs();
        self.put_in_tx_mode();

        // clear related events
        regs.events_phyend().write_value(0);
        regs.events_end().write_value(0);

        // NOTE(unsafe) DMA transfer has not yet started
        regs.packetptr().write_value(packet.buffer.as_ptr() as u32);

        // configure radio to disable transmitter once packet is sent
        regs.shorts().modify(|w| w.set_end_disable(true));

        // start DMA transfer
        dma_start_fence();
        regs.tasks_start().write_value(1);

        self.wait_for_event(Event::PhyEnd);
        regs.shorts().write_value(Shorts::default());
    }

    /// Moves the radio from any state to the DISABLED state
    pub fn disable(&mut self) {
        let regs = self.regs();
        // See figure 110 in nRF52840-PS
        loop {
            match regs.state().read().state() {
                RadioState::DISABLED => return,
                RadioState::RX_RU
                | RadioState::RX_IDLE
                | RadioState::TX_RU
                | RadioState::TX_IDLE => {
                    regs.tasks_disable().write_value(1);

                    self.wait_for_state_a(RadioState::DISABLED);
                    return;
                }
                RadioState::RX_DISABLE | RadioState::TX_DISABLE => {
                    self.wait_for_state_a(RadioState::DISABLED);
                    return;
                }
                RadioState::RX => {
                    regs.tasks_ccastop().write_value(1);
                    regs.tasks_stop().write_value(1);
                    self.wait_for_state_a(RadioState::RX_IDLE);
                }
                RadioState::TX => {
                    regs.tasks_stop().write_value(1);
                    self.wait_for_state_a(RadioState::TX_IDLE);
                }
                _ => (),
            }
        }
    }

    /// Moves the radio to the RXIDLE state
    fn put_in_rx_mode(&mut self) {
        let regs = self.regs();
        let state = self.state();

        let (disable, enable) = match state {
            State::Disabled => (false, true),
            State::RxIdle => (false, self.needs_enable),
            // NOTE to avoid errata 204 (see rev1 v1.4) we do TXIDLE -> DISABLED -> RXIDLE
            State::TxIdle => (true, true),
        };

        if disable {
            regs.tasks_disable().write_value(1);
            self.wait_for_state_a(RadioState::DISABLED);
        }

        if enable {
            self.needs_enable = false;
            regs.tasks_rxen().write_value(1);
            self.wait_for_state_a(RadioState::RX_IDLE);
        }
    }

    /// Moves the radio to the TXIDLE state
    fn put_in_tx_mode(&mut self) {
        let state = self.state();

        if state != State::TxIdle || self.needs_enable {
            self.needs_enable = false;
            self.regs().tasks_rxen().write_value(1);
            self.wait_for_state_a(RadioState::TX_IDLE);
        }
    }

    fn wait_for_event(&self, event: Event) {
        match event {
            Event::PhyEnd => {
                while self.regs().events_phyend().read() == 0 {}
                self.regs().events_phyend().write_value(0);
            }
        }
    }

    //use hal::pac::radio::vals::State
    /// Waits until the radio state matches the given `state`
    fn wait_for_state_a(&self, state: RadioState) {
        while self.regs().state().read().state() != state {}
    }
}

/// An IEEE 802.15.4 packet
///
/// This `Packet` is a PHY layer packet. It's made up of the physical header (PHR) and the PSDU
/// (PHY service data unit). The PSDU of this `Packet` will always include the MAC level CRC, AKA
/// the FCS (Frame Control Sequence) -- the CRC is fully computed in hardware and automatically
/// appended on transmission and verified on reception.
///
/// The API lets users modify the usable part (not the CRC) of the PSDU via the `deref` and
/// `copy_from_slice` methods. These methods will automatically update the PHR.
///
/// See figure 119 in the Product Specification of the nRF52840 for more details
pub struct Packet {
    buffer: [u8; Self::SIZE],
}

// See figure 124 in nRF52840-PS
impl Packet {
    // for indexing purposes
    const PHY_HDR: usize = 0;
    const DATA: core::ops::RangeFrom<usize> = 1..;

    /// Maximum amount of usable payload (CRC excluded) a single packet can contain, in bytes
    pub const CAPACITY: u8 = 125;
    const CRC: u8 = 2; // size of the CRC, which is *never* copied to / from RAM
    const MAX_PSDU_LEN: u8 = Self::CAPACITY + Self::CRC;
    const SIZE: usize = 1 /* PHR */ + Self::MAX_PSDU_LEN as usize;

    /// Returns an empty packet (length = 0)
    pub fn new() -> Self {
        let mut packet = Self {
            buffer: [0; Self::SIZE],
        };
        packet.set_len(0);
        packet
    }

    /// Fills the packet payload with given `src` data
    ///
    /// # Panics
    ///
    /// This function panics if `src` is larger than `Self::CAPACITY`
    pub fn copy_from_slice(&mut self, src: &[u8]) {
        assert!(src.len() <= Self::CAPACITY as usize);
        let len = src.len() as u8;
        self.buffer[Self::DATA][..len as usize].copy_from_slice(&src[..len.into()]);
        self.set_len(len);
    }

    /// Returns the size of this packet's payload
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> u8 {
        self.buffer[Self::PHY_HDR] - Self::CRC
    }

    /// Changes the size of the packet's payload
    ///
    /// # Panics
    ///
    /// This function panics if `len` is larger than `Self::CAPACITY`
    pub fn set_len(&mut self, len: u8) {
        assert!(len <= Self::CAPACITY);
        self.buffer[Self::PHY_HDR] = len + Self::CRC;
    }

    /// Returns the LQI (Link Quality Indicator) of the received packet
    ///
    /// Note that the LQI is stored in the `Packet`'s internal buffer by the hardware so the value
    /// returned by this method is only valid after a `Radio.recv` operation. Operations that
    /// modify the `Packet`, like `copy_from_slice` or `set_len`+`deref_mut`, will overwrite the
    /// stored LQI value.
    ///
    /// Also note that the hardware will *not* compute a LQI for packets smaller than 3 bytes so
    /// this method will return an invalid value for those packets.
    pub fn lqi(&self) -> u8 {
        self.buffer[1 /* PHY_HDR */ + self.len() as usize /* data */]
    }
}

impl Default for Packet {
    fn default() -> Self {
        Self::new()
    }
}

impl core::ops::Deref for Packet {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.buffer[Self::DATA][..self.len() as usize]
    }
}

impl core::ops::DerefMut for Packet {
    fn deref_mut(&mut self) -> &mut [u8] {
        let len = self.len();
        &mut self.buffer[Self::DATA][..len as usize]
    }
}

/// NOTE must be followed by a volatile write operation
fn dma_start_fence() {
    compiler_fence(Ordering::Release);
}

/// NOTE must be preceded by a volatile read operation
fn dma_end_fence() {
    compiler_fence(Ordering::Acquire);
}

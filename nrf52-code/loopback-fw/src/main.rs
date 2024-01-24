//! Firmware for the nRF52840 Dongle, for echoing packets in loopback mode
//!
//! Sets up a USB Serial port and listens for radio packets.

#![no_std]
#![no_main]

use core::fmt::Write;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use cortex_m_rt::entry;
use usb_device::class_prelude::UsbBusAllocator;
use usb_device::device::{UsbDevice, UsbDeviceBuilder, UsbVidPid};
use usbd_hid::hid_class::HIDClass;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use dongle::peripheral::interrupt;
use dongle::{
    hal::usbd,
    ieee802154::{Channel, Packet},
};

/// A 64-byte USB Serial buffer
static RING_BUFFER: Ringbuffer = Ringbuffer {
    buffer: heapless::mpmc::Q64::new(),
    force_buffer: AtomicBool::new(true),
};

/// A short-hand for the nRF52 USB types
type UsbBus<'a> = usbd::Usbd<usbd::UsbPeripheral<'a>>;

/// The USB Device Driver (owned by the USBD interrupt).
static mut USB_DEVICE: Option<UsbDevice<UsbBus>> = None;

/// The USB Bus Driver (owned by the USBD interrupt).
static mut USB_BUS: Option<UsbBusAllocator<UsbBus>> = None;

/// The USB Serial Device Driver (owned by the USBD interrupt).
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

/// The USB Human Interface Device Driver (owned by the USBD interrupt).
static mut USB_HID: Option<HIDClass<UsbBus>> = None;

/// Track how many CRC successes we had receiving radio packets
static RX_COUNT: AtomicU32 = AtomicU32::new(0);

/// Track how many CRC failures we had receiving radio packets
static ERR_COUNT: AtomicU32 = AtomicU32::new(0);

/// The USB interrupt sets this to < u32::MAX when a new channel is sent over HID.
///
/// The main loop handles it and sets it back to u32::MAX when processed.
static NEW_CHANNEL: AtomicU32 = AtomicU32::new(u32::MAX);

/// Set to true when we get a ?.
///
/// We print some info in response.
static WANT_INFO: AtomicBool = AtomicBool::new(false);

/// Is the UART connected?
static IS_CONNECTED: AtomicBool = AtomicBool::new(false);

struct Ringbuffer {
    buffer: heapless::mpmc::Q64<u8>,
    force_buffer: AtomicBool,
}

impl core::fmt::Write for &Ringbuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            if IS_CONNECTED.load(Ordering::Relaxed) || self.force_buffer.load(Ordering::Relaxed) {
                // spin until space in the UART
                while let Err(_) = self.buffer.enqueue(b) {
                    cortex_m::asm::wfi();
                }
            } else {
                // drop the data - we're not connected
            }
        }
        Ok(())
    }
}

#[entry]
fn main() -> ! {
    let mut board = dongle::init().unwrap();
    board.usbd.inten.modify(|_r, w| {
        w.sof().set_bit();
        w
    });
    let usb_bus = UsbBusAllocator::new(usbd::Usbd::new(usbd::UsbPeripheral::new(
        board.usbd,
        board.clocks,
    )));
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_BUS = Some(usb_bus);
    }
    // Grab a reference to the USB Bus allocator. We are promising to the
    // compiler not to take mutable access to this global variable whilst this
    // reference exists!
    let bus_ref = unsafe { USB_BUS.as_ref().unwrap() };
    let serial = SerialPort::new(bus_ref);
    unsafe {
        USB_SERIAL = Some(serial);
    }

    let desc = &[
        0x06, 0x00, 0xFF, // Item(Global): Usage Page, data= [ 0x00 0xff ] 65280
        0x09, 0x01, // Item(Local ): Usage, data= [ 0x01 ] 1
        0xA1, 0x01, // Item(Main  ): Collection, data= [ 0x01 ] 1
        //               Application
        0x15, 0x00, // Item(Global): Logical Minimum, data= [ 0x00 ] 0
        0x26, 0xFF, 0x00, // Item(Global): Logical Maximum, data= [ 0xff 0x00 ] 255
        0x75, 0x08, // Item(Global): Report Size, data= [ 0x08 ] 8
        0x95, 0x40, // Item(Global): Report Count, data= [ 0x40 ] 64
        0x09, 0x01, // Item(Local ): Usage, data= [ 0x01 ] 1
        0x81, 0x02, // Item(Main  ): Input, data= [ 0x02 ] 2
        //               Data Variable Absolute No_Wrap Linear
        //               Preferred_State No_Null_Position Non_Volatile Bitfield
        0x95, 0x40, // Item(Global): Report Count, data= [ 0x40 ] 64
        0x09, 0x01, // Item(Local ): Usage, data= [ 0x01 ] 1
        0x91, 0x02, // Item(Main  ): Output, data= [ 0x02 ] 2
        //               Data Variable Absolute No_Wrap Linear
        //               Preferred_State No_Null_Position Non_Volatile Bitfield
        0x95, 0x01, // Item(Global): Report Count, data= [ 0x01 ] 1
        0x09, 0x01, // Item(Local ): Usage, data= [ 0x01 ] 1
        0xB1, 0x02, // Item(Main  ): Feature, data= [ 0x02 ] 2
        //               Data Variable Absolute No_Wrap Linear
        //               Preferred_State No_Null_Position Non_Volatile Bitfield
        0xC0, // Item(Main  ): End Collection, data=none
    ];
    let hid = HIDClass::new(bus_ref, desc, 100);
    unsafe {
        USB_HID = Some(hid);
    }

    let vid_pid = UsbVidPid(consts::USB_VID_DEMO, consts::USB_PID_DONGLE_PUZZLE);
    let usb_dev = UsbDeviceBuilder::new(bus_ref, vid_pid)
        .manufacturer("Ferrous Systems")
        .product("Dongle Puzzle")
        .device_class(USB_CLASS_CDC)
        .max_packet_size_0(64) // (makes control transfers 8x faster)
        .build();
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_DEVICE = Some(usb_dev);
    }

    let mut current_ch_id = 25;
    board.radio.set_channel(dongle::ieee802154::Channel::_25);

    // Turn on USB interrupts...
    unsafe {
        cortex_m::peripheral::NVIC::unmask(dongle::peripheral::Interrupt::USBD);
    };

    let _ = writeln!(
        &RING_BUFFER,
        "deviceid={:08x}{:08x} channel={} TxPower=+8dBm app=loopback-fw",
        dongle::deviceid1(),
        dongle::deviceid0(),
        current_ch_id
    );

    // Now the sign-up message is done, turn off force buffering
    RING_BUFFER.force_buffer.store(false, Ordering::Relaxed);

    board.leds.ld1.on();
    board.leds.ld2_blue.on();
    let mut pkt = Packet::new();
    loop {
        // Wait up to 1 second for a radio packet
        match board
            .radio
            .recv_timeout(&mut pkt, &mut board.timer, 1_000_000)
        {
            Ok(crc) => {
                board.leds.ld1.toggle();
                let _ = writeln!(
                    &RING_BUFFER,
                    "received {} bytes (CRC=Ok(0x{:04x}), LQI={})",
                    pkt.len(),
                    crc,
                    pkt.lqi()
                );
                // now send it back
                board.radio.send(&mut pkt);
                RX_COUNT.fetch_add(1, Ordering::Relaxed);
            }
            Err(dongle::ieee802154::Error::Crc(_)) => {
                ERR_COUNT.fetch_add(1, Ordering::Relaxed);
            }
            Err(dongle::ieee802154::Error::Timeout) => {
                // do nothing
            }
        }

        // Handle channel changes
        let ch_id = NEW_CHANNEL.load(Ordering::Relaxed);
        if ch_id != u32::MAX {
            NEW_CHANNEL.store(u32::MAX, Ordering::Relaxed);
            if let Some(channel) = match ch_id {
                11 => Some(Channel::_11),
                12 => Some(Channel::_12),
                13 => Some(Channel::_13),
                14 => Some(Channel::_14),
                15 => Some(Channel::_15),
                16 => Some(Channel::_16),
                17 => Some(Channel::_17),
                18 => Some(Channel::_18),
                19 => Some(Channel::_19),
                20 => Some(Channel::_20),
                21 => Some(Channel::_21),
                22 => Some(Channel::_22),
                23 => Some(Channel::_23),
                24 => Some(Channel::_24),
                25 => Some(Channel::_25),
                26 => Some(Channel::_26),
                _ => None,
            } {
                board.radio.set_channel(channel);
                let _ = writeln!(&RING_BUFFER, "now listening on channel {}", ch_id);
                current_ch_id = ch_id;
            } else {
                let _ = writeln!(&RING_BUFFER, "Channel {} invalid", ch_id);
            }
        }

        let connected = IS_CONNECTED.load(Ordering::Relaxed);
        if connected {
            board.leds.ld2_red.on();
        } else {
            board.leds.ld2_red.off();
        }

        // Print help text when ? is pressed
        if WANT_INFO.load(Ordering::Relaxed) {
            WANT_INFO.store(false, Ordering::Relaxed);
            let _ = writeln!(
                &RING_BUFFER,
                "rx={}, err={}, ch={}, app=loopback-fw",
                RX_COUNT.load(Ordering::Relaxed),
                ERR_COUNT.load(Ordering::Relaxed),
                current_ch_id,
            );
        }
    }
}

/// Handles USB interrupts
///
/// Polls all the USB devices, and copies bytes from [`RING_BUFFER`] into the
/// USB UART.
#[interrupt]
fn USBD() {
    // Grab the global objects. This is OK as we only access them under interrupt.
    let usb_dev = unsafe { USB_DEVICE.as_mut().unwrap() };
    let serial = unsafe { USB_SERIAL.as_mut().unwrap() };
    let hid = unsafe { USB_HID.as_mut().unwrap() };

    let mut buf = [0u8; 64];

    // Poll the USB driver with all of our supported USB Classes
    if usb_dev.poll(&mut [serial, hid]) {
        match serial.read(&mut buf) {
            Err(_e) => {
                // Do nothing
            }
            Ok(0) => {
                // Do nothing
            }
            Ok(count) => {
                for item in &buf[0..count] {
                    // Look for question marks
                    if *item == b'?' {
                        WANT_INFO.store(true, Ordering::Relaxed);
                    }
                }
            }
        }
        let hid_byte = match hid.pull_raw_output(&mut buf) {
            Ok(64) => {
                // Windows zero-pads the packet
                Some(buf[0])
            }
            Ok(1) => {
                // macOS/Linux sends a single byte
                Some(buf[0])
            }
            Ok(_n) => {
                // Ignore any other size packet
                None
            }
            Err(_e) => None,
        };
        if let Some(ch) = hid_byte {
            NEW_CHANNEL.store(ch as u32, Ordering::Relaxed);
        }
    }

    // Copy from ring-buffer to USB UART
    let mut count = 0;
    while count < buf.len() {
        if let Some(item) = RING_BUFFER.buffer.dequeue() {
            buf[count] = item;
            count += 1;
        } else {
            break;
        }
    }
    let _ = serial.write(&buf[0..count]);
    IS_CONNECTED.store(serial.dtr(), Ordering::Relaxed);
    cortex_m::asm::sev();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let _ = writeln!(&RING_BUFFER, "Panic: {:?}", info);
    cortex_m::asm::delay(64_000_000 * 2);
    unsafe {
        loop {
            core::arch::asm!("bkpt 0x00");
        }
    }
}

// End of file

//! Firmware for the nRF52840 Dongle, for playing the puzzle game
//!
//! Sets up a USB Serial port and listens for radio packets.

#![no_std]
#![no_main]

use core::fmt::Write;
use core::sync::atomic::{AtomicU32, Ordering};

use cortex_m_rt::entry;
use panic_probe as _;
use usb_device::class_prelude::UsbBusAllocator;
use usb_device::device::{UsbDevice, UsbDeviceBuilder, UsbVidPid};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use dongle::peripheral::interrupt;
use dongle::{hal::usbd, ieee802154::Packet};

/// Store the secret.
///
/// We do this rather than the plaintext -- otherwise `strings $elf` will reveal the answer
static SECRET: &[u8] = env!("SECRET_MESSAGE").as_bytes();

/// The plaintext side of the map
static PLAIN_LETTERS: &[u8] = env!("PLAIN_LETTERS").as_bytes();

/// The ciphertext side of the map
static CIPHER_LETTERS: &[u8] = env!("CIPHER_LETTERS").as_bytes();

/// A 64-byte USB Serial buffer
static RING_BUFFER: Ringbuffer = Ringbuffer {
    buffer: heapless::mpmc::Q64::new(),
};

/// The USB Device Driver (shared with the interrupt).
static mut USB_DEVICE: Option<UsbDevice<usbd::Usbd<usbd::UsbPeripheral>>> = None;

/// The USB Bus Driver (shared with the interrupt).
static mut USB_BUS: Option<UsbBusAllocator<usbd::Usbd<usbd::UsbPeripheral>>> = None;

/// The USB Serial Device Driver (shared with the interrupt).
static mut USB_SERIAL: Option<SerialPort<usbd::Usbd<usbd::UsbPeripheral>>> = None;

static RX_COUNT: AtomicU32 = AtomicU32::new(0);

static ERR_COUNT: AtomicU32 = AtomicU32::new(0);

struct Ringbuffer {
    buffer: heapless::mpmc::Q64<u8>,
}

impl core::fmt::Write for &Ringbuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            let _ = self.buffer.enqueue(b);
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

    board.radio.set_channel(dongle::ieee802154::Channel::_25);

    let mut dict: heapless::LinearMap<u8, u8, 128> = heapless::LinearMap::new();
    for (&plain, &cipher) in PLAIN_LETTERS.iter().zip(CIPHER_LETTERS.iter()) {
        let _ = dict.insert(plain, cipher);
    }

    // Turn on USB interrupts...
    // Enable the USB interrupt
    unsafe {
        cortex_m::peripheral::NVIC::unmask(dongle::peripheral::Interrupt::USBD);
    };

    let mut pkt = Packet::new();
    loop {
        board.leds.ld1.on();
        match board.radio.recv(&mut pkt) {
            Ok(crc) => {
                board.leds.ld1.off();
                write!(&RING_BUFFER, "RX CRC {crc:04x}, LQI ").unwrap();
                if pkt.len() > 3 {
                    let lqi = pkt.lqi();
                    writeln!(&RING_BUFFER, "{lqi}").unwrap();
                } else {
                    writeln!(&RING_BUFFER, "Unknown").unwrap();
                }

                match handle_packet(&pkt, &dict) {
                    Command::SendSecret => {
                        pkt.copy_from_slice(&SECRET);
                        writeln!(&RING_BUFFER, "TX Secret").unwrap();
                        board.leds.ld2_blue.on();
                        board.leds.ld2_green.off();
                        board.leds.ld2_red.off();
                    }
                    Command::MapChar(from, to) => {
                        pkt.copy_from_slice(&[to]);
                        writeln!(&RING_BUFFER, "TX Map({from}) => {to}").unwrap();
                        board.leds.ld2_blue.off();
                        board.leds.ld2_green.on();
                        board.leds.ld2_red.off();
                    }
                    Command::Correct => {
                        pkt.copy_from_slice(b"correct");
                        writeln!(&RING_BUFFER, "TX Correct").unwrap();
                        board.leds.ld2_blue.on();
                        board.leds.ld2_green.on();
                        board.leds.ld2_red.on();
                    }
                    Command::Wrong => {
                        pkt.copy_from_slice(b"incorrect");
                        writeln!(&RING_BUFFER, "TX Incorrect").unwrap();
                        board.leds.ld2_blue.off();
                        board.leds.ld2_green.on();
                        board.leds.ld2_red.on();
                    }
                }
                // wait 1ms so they have time to switch to receive mode
                board.timer.delay(1000);
                // now send it
                board.radio.send(&mut pkt);
                RX_COUNT.fetch_add(1, Ordering::Relaxed);
            }
            Err(_crc) => {
                ERR_COUNT.fetch_add(1, Ordering::Relaxed);
            }
        }
    }
}

enum Command {
    SendSecret,
    MapChar(u8, u8),
    Correct,
    Wrong,
}

fn handle_packet(packet: &Packet, dict: &heapless::LinearMap<u8, u8, 128>) -> Command {
    if packet.len() == 0 {
        Command::SendSecret
    } else if packet.len() == 1 {
        // They want to know how convert from X to Y, and they gave us X
        let from = packet[0];
        let to = *dict.get(&from).unwrap_or(&0);
        Command::MapChar(from, to)
    } else {
        let mut correct = packet.len() as usize == SECRET.len();
        // Encrypt every byte of plaintext they give us
        for (encrypted, secret) in packet
            .iter()
            .map(|b| dict.get(b).unwrap_or(&0))
            .zip(SECRET.iter())
        {
            if secret != encrypted {
                correct = false;
            }
        }

        if correct {
            Command::Correct
        } else {
            Command::Wrong
        }
    }
}

#[interrupt]
unsafe fn USBD() {
    // Grab the global objects. This is OK as we only access them under interrupt.
    let usb_dev = USB_DEVICE.as_mut().unwrap();
    let serial = USB_SERIAL.as_mut().unwrap();

    let mut buf = [0u8; 64];

    // Poll the USB driver with all of our supported USB Classes
    if usb_dev.poll(&mut [serial]) {
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
                        let _ = writeln!(
                            &RING_BUFFER,
                            "{}, {}",
                            RX_COUNT.load(Ordering::Relaxed),
                            ERR_COUNT.load(Ordering::Relaxed)
                        );
                    }
                }
            }
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
}

// End of file

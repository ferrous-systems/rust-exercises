//! Firmware for the nRF52840 Dongle, for echoing packets in loopback mode
//!
//! Sets up a USB Serial port and listens for radio packets.

#![no_main]
#![no_std]

use defmt_rtt as _;

#[rtic::app(device = dongle, peripherals = false)]
mod app {
    use core::mem::MaybeUninit;
    use rtic_monotonics::systick::prelude::*;
    const QUEUE_LEN: usize = 8;

    systick_monotonic!(Mono, 100);

    /// An adapter that lets us writeln! into any closure that takes a byte.
    ///
    /// This is useful if writing a byte requires taking a lock, and you don't
    /// want to hold the lock for the duration of the write.
    struct Writer<F>(F)
    where
        F: FnMut(&[u8]);

    impl<F> core::fmt::Write for Writer<F>
    where
        F: FnMut(&[u8]),
    {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            (self.0)(s.as_bytes());
            Ok(())
        }
    }

    #[local]
    struct MyLocalResources {
        /// The radio subsystem
        radio: dongle::ieee802154::Radio<'static>,
        /// Which channel are we on
        current_channel: u8,
        /// Holds one package, for receive or transmit
        packet: dongle::ieee802154::Packet,
        /// Used to measure elapsed time
        timer: dongle::Timer,
        /// How many packets have been received OK?
        rx_count: u32,
        /// How many packets have been received with errors?
        err_count: u32,
        /// A place to read the message queue
        msg_queue_out: heapless::spsc::Consumer<'static, Message, QUEUE_LEN>,
        /// A place to write to the message queue
        msg_queue_in: heapless::spsc::Producer<'static, Message, QUEUE_LEN>,
        /// A status LED
        led: dongle::Led,
    }

    #[derive(Debug, defmt::Format, Copy, Clone, PartialEq, Eq)]
    enum Message {
        ChangeChannel(u8),
        WantInfo,
    }

    #[shared]
    struct MySharedResources {
        /// Handles the USB Serial interface, including a ring buffer
        usb_serial: usbd_serial::SerialPort<'static, dongle::UsbBus>,
        /// Handles the USB HID interface
        usb_hid: usbd_hid::hid_class::HIDClass<'static, dongle::UsbBus>,
        /// Handles the lower-level USB Device interface
        usb_device: usb_device::device::UsbDevice<'static, dongle::UsbBus>,
    }

    #[init(local = [
        usb_alloc: MaybeUninit<usb_device::bus::UsbBusAllocator<dongle::UsbBus>> = MaybeUninit::uninit(),
        queue: heapless::spsc::Queue<Message, QUEUE_LEN> = heapless::spsc::Queue::new(),
    ])]
    fn init(ctx: init::Context) -> (MySharedResources, MyLocalResources) {
        let mut board = dongle::init().unwrap();
        Mono::start(ctx.core.SYST, 64_000_000);

        defmt::debug!("Enabling interrupts...");
        board.usbd.inten.modify(|_r, w| {
            w.sof().set_bit();
            w
        });

        defmt::debug!("Building USB allocator...");
        let usbd = dongle::UsbBus::new(dongle::hal::usbd::UsbPeripheral::new(
            board.usbd,
            board.clocks,
        ));
        let usb_alloc = ctx
            .local
            .usb_alloc
            .write(usb_device::bus::UsbBusAllocator::new(usbd));

        defmt::debug!("Creating usb_serial...");
        let usb_serial = usbd_serial::SerialPort::new(usb_alloc);

        defmt::debug!("Creating usb_hid...");
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
        let usb_hid = usbd_hid::hid_class::HIDClass::new(usb_alloc, desc, 100);

        defmt::debug!("Building USB Strings...");
        let strings = usb_device::device::StringDescriptors::new(usb_device::LangID::EN)
            .manufacturer("Ferrous Systems")
            .product("Test Device");

        defmt::debug!("Building VID and PID...");
        let vid_pid =
            usb_device::device::UsbVidPid(consts::USB_VID_DEMO, consts::USB_PID_DONGLE_LOOPBACK);

        defmt::debug!("Building USB Device...");
        let usb_device = usb_device::device::UsbDeviceBuilder::new(usb_alloc, vid_pid)
            .composite_with_iads()
            .strings(&[strings])
            .expect("Adding strings")
            .max_packet_size_0(64)
            .expect("set_packet_size")
            .build();

        defmt::debug!("Configuring radio...");
        board.radio.set_channel(dongle::ieee802154::Channel::_20);
        let current_channel = 20;

        let (msg_queue_in, msg_queue_out) = ctx.local.queue.split();

        defmt::debug!("Building structures...");
        let shared = MySharedResources {
            usb_serial,
            usb_hid,
            usb_device,
        };
        let local = MyLocalResources {
            radio: board.radio,
            current_channel,
            packet: dongle::ieee802154::Packet::new(),
            timer: board.timer,
            rx_count: 0,
            err_count: 0,
            msg_queue_out,
            msg_queue_in,
            led: board.leds.ld1,
        };

        defmt::debug!("Init Complete!");

        (shared, local)
    }

    #[idle(local = [radio, current_channel, packet, timer, rx_count, err_count, msg_queue_out, led], shared = [usb_serial])]
    fn idle(mut ctx: idle::Context) -> ! {
        use core::fmt::Write as _;
        let mut writer = Writer(|b: &[u8]| {
            ctx.shared.usb_serial.lock(|usb_serial| {
                let _ = usb_serial.write(b);
            })
        });

        defmt::info!(
            "deviceid={=u32:08x}{=u32:08x} channel={=u8} TxPower=+8dBm app=loopback-fw",
            dongle::deviceid1(),
            dongle::deviceid0(),
            ctx.local.current_channel
        );

        loop {
            while let Some(msg) = ctx.local.msg_queue_out.dequeue() {
                match msg {
                    Message::WantInfo => {
                        defmt::info!(
                            "rx={=u32}, err={=u32}, ch={=u8}, app=loopback-fw",
                            ctx.local.rx_count,
                            ctx.local.err_count,
                            ctx.local.current_channel
                        );
                        let _ = writeln!(
                            writer,
                            "\nrx={}, err={}, ch={}, app=loopback-fw",
                            ctx.local.rx_count, ctx.local.err_count, ctx.local.current_channel
                        );
                    }
                    Message::ChangeChannel(n) => {
                        defmt::info!("Changing Channel to {}", n);
                        let _ = writeln!(writer, "\nChanging Channel to {}", n);
                        match n {
                            11 => {
                                ctx.local
                                    .radio
                                    .set_channel(dongle::ieee802154::Channel::_11);
                            }
                            12 => {
                                ctx.local
                                    .radio
                                    .set_channel(dongle::ieee802154::Channel::_12);
                            }
                            13 => {
                                ctx.local
                                    .radio
                                    .set_channel(dongle::ieee802154::Channel::_13);
                            }
                            14 => {
                                ctx.local
                                    .radio
                                    .set_channel(dongle::ieee802154::Channel::_14);
                            }
                            15 => {
                                ctx.local
                                    .radio
                                    .set_channel(dongle::ieee802154::Channel::_15);
                            }
                            16 => {
                                ctx.local
                                    .radio
                                    .set_channel(dongle::ieee802154::Channel::_16);
                            }
                            17 => {
                                ctx.local
                                    .radio
                                    .set_channel(dongle::ieee802154::Channel::_17);
                            }
                            18 => {
                                ctx.local
                                    .radio
                                    .set_channel(dongle::ieee802154::Channel::_18);
                            }
                            19 => {
                                ctx.local
                                    .radio
                                    .set_channel(dongle::ieee802154::Channel::_19);
                            }
                            20 => {
                                ctx.local
                                    .radio
                                    .set_channel(dongle::ieee802154::Channel::_20);
                            }
                            21 => {
                                ctx.local
                                    .radio
                                    .set_channel(dongle::ieee802154::Channel::_21);
                            }
                            22 => {
                                ctx.local
                                    .radio
                                    .set_channel(dongle::ieee802154::Channel::_22);
                            }
                            23 => {
                                ctx.local
                                    .radio
                                    .set_channel(dongle::ieee802154::Channel::_23);
                            }
                            24 => {
                                ctx.local
                                    .radio
                                    .set_channel(dongle::ieee802154::Channel::_24);
                            }
                            25 => {
                                ctx.local
                                    .radio
                                    .set_channel(dongle::ieee802154::Channel::_25);
                            }
                            26 => {
                                ctx.local
                                    .radio
                                    .set_channel(dongle::ieee802154::Channel::_26);
                            }
                            _ => {
                                defmt::info!("Bad Channel {}!", n);
                            }
                        }
                    }
                }
            }

            defmt::debug!("Waiting for packet..");
            match ctx.local.radio.recv_timeout(
                &mut ctx.local.packet,
                &mut ctx.local.timer,
                1_000_000,
            ) {
                Ok(crc) => {
                    ctx.local.led.toggle();
                    defmt::info!(
                        "Received {=u8} bytes (CRC=0x{=u16:04x}, LQI={})",
                        ctx.local.packet.len(),
                        crc,
                        ctx.local.packet.lqi(),
                    );
                    let _ = writeln!(
                        writer,
                        "\nReceived {} bytes (CRC=0x{:04x}, LQI={})",
                        ctx.local.packet.len(),
                        crc,
                        ctx.local.packet.lqi(),
                    );
                    *ctx.local.rx_count += 1;
                    // reverse the bytes, so olleh -> hello
                    ctx.local.packet.reverse();
                    // send packet after 5ms (we know the client waits for 10ms and
                    // we want to ensure they are definitely in receive mode by the
                    // time we send this reply)
                    ctx.local.timer.delay(5000);
                    ctx.local.radio.send(ctx.local.packet);
                }
                Err(dongle::ieee802154::Error::Crc(_)) => {
                    defmt::debug!("RX fail!");
                    let _ = write!(writer, "!");
                    *ctx.local.err_count += 1;
                }
                Err(dongle::ieee802154::Error::Timeout) => {
                    defmt::debug!("RX timeout...");
                    let _ = write!(writer, ".");
                }
            }
        }
    }

    /// USB Interrupt Handler
    ///
    /// USB Device is set to fire this whenever there's a Start of Frame from
    /// the USB Host.
    #[task(binds = USBD, local = [msg_queue_in], shared = [usb_serial, usb_hid, usb_device])]
    fn usb_isr(mut ctx: usb_isr::Context) {
        let mut all = (
            &mut ctx.shared.usb_serial,
            &mut ctx.shared.usb_hid,
            &mut ctx.shared.usb_device,
        );
        all.lock(|usb_serial, usb_hid, usb_device| {
            if usb_device.poll(&mut [usb_serial, usb_hid]) {
                let mut buffer = [0u8; 64];
                if let Ok(n) = usb_serial.read(&mut buffer) {
                    if n > 0 {
                        for b in &buffer[0..n] {
                            if *b == b'?' {
                                // User pressed "?" in the terminal
                                _ = ctx.local.msg_queue_in.enqueue(Message::WantInfo);
                            }
                        }
                    }
                }
                if let Ok(n) = usb_hid.pull_raw_output(&mut buffer) {
                    // Linux sends 1 byte, Windows sends 64 (with 63 zero bytes)
                    if n == 1 || n == 64 {
                        _ = ctx
                            .local
                            .msg_queue_in
                            .enqueue(Message::ChangeChannel(buffer[0]));
                    }
                }
            }
        });
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        defmt::error!("Panic at {}:{}", location.file(), location.line());
    } else {
        defmt::error!("Panic at unknown location");
    }
    loop {
        core::hint::spin_loop();
    }
}

defmt::timestamp!("{=u64:tus}", dongle::uptime_us());

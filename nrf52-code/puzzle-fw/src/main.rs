//! Firmware for the nRF52840 Dongle, for playing the puzzle game
//!
//! Sets up a USB Serial port and listens for radio packets.

#![no_main]
#![no_std]

#[cfg(not(feature = "dk"))]
use bsp::RgbLed;

#[rtic::app(device = bsp, peripherals = false, dispatchers = [QSPI, CRYPTOCELL])]
mod app {
    use bsp::hal::{self, usb::vbus_detect::HardwareVbusDetect};
    use core::fmt::Write as _;
    use defmt_rtt as _;
    use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
    use embassy_usb::class::cdc_acm;
    use embassy_usb::class::hid;
    use rtic_monotonics::fugit::ExtU32;

    use rtic_monotonics::systick::prelude::*;
    use static_cell::StaticCell;

    const MSG_QUEUE_LEN: usize = 8;
    const ACM_QUEUE_LEN: usize = 128;
    const MAX_ACM_PACKET_SIZE: usize = 64;

    /// The secret message, but encoded.
    ///
    /// We do this rather than the plaintext -- otherwise `strings $elf` will reveal the answer
    static ENCODED_MESSAGE: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/ENCODED_MESSAGE.txt"));

    /// The plaintext side of the map
    static PLAIN_LETTERS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/PLAIN_LETTERS.txt"));

    /// The ciphertext side of the map
    static CIPHER_LETTERS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/CIPHER_LETTERS.txt"));

    /// How many address bytes we reflect back
    const ADDR_BYTES: usize = 6;

    hal::bind_interrupts!(struct Irqs {
        USBD => hal::usb::InterruptHandler<hal::peripherals::USBD>;
        CLOCK_POWER => hal::usb::vbus_detect::InterruptHandler;
        #[cfg(feature = "dk")]
        RADIO => hal::radio::InterruptHandler<hal::peripherals::RADIO>;
    });

    systick_monotonic!(Mono);

    /// An adapter that simplifies asynchronously writing to the USB ACM by buffering writes.
    ///
    /// All writes are buffered until `flush` is called, which performs the async write.
    struct WriteAsyncPipeAdapter {
        // Intermediate buffer which is required because we can not used async code
        // in the [core::fmt::Write] implementation.
        buffer: heapless::String<256>,
        usb_acm_writer: embassy_sync::pipe::Writer<'static, CriticalSectionRawMutex, ACM_QUEUE_LEN>,
    }

    impl core::fmt::Write for WriteAsyncPipeAdapter {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            write!(self.buffer, "{}", s).unwrap();
            Ok(())
        }
    }

    impl WriteAsyncPipeAdapter {
        /// Flush the buffer to the underlying writer.
        async fn flush(&mut self) {
            self.usb_acm_writer.write(self.buffer.as_bytes()).await;
            self.buffer.clear();
        }
    }

    #[derive(Debug, defmt::Format, Copy, Clone, PartialEq, Eq)]
    enum Message {
        ChangeChannel(u8),
        WantInfo,
    }

    struct HidTransferHandler(
        embassy_sync::channel::Sender<'static, CriticalSectionRawMutex, Message, MSG_QUEUE_LEN>,
    );

    impl hid::RequestHandler for HidTransferHandler {
        // HID requests are used to switch the channel.
        fn set_report(
            &mut self,
            _report_id: hid::ReportId,
            data: &[u8],
        ) -> embassy_usb::control::OutResponse {
            // Linux sends 1 byte, Windows sends 64 (with 63 zero bytes)
            if data.len() == 1 || data.len() == 64 {
                let _ = self.0.try_send(Message::ChangeChannel(data[0]));
            }
            embassy_usb::control::OutResponse::Accepted
        }
    }

    #[local]
    struct MyLocalResources {
        /// The radio subsystem
        radio: bsp::hal::radio::ieee802154::Radio<'static>,
        /// Which channel are we on
        current_channel: u8,
        /// Holds one package, for receive or transmit
        packet: bsp::hal::radio::ieee802154::Packet,
        /// Used to measure elapsed time
        timer: bsp::Timer,
        /// How many packets have been received OK?
        rx_count: u32,
        /// How many packets have been received with errors?
        err_count: u32,
        /// A place to read the message queue
        msg_queue_rx: embassy_sync::channel::Receiver<
            'static,
            CriticalSectionRawMutex,
            Message,
            MSG_QUEUE_LEN,
        >,
        /// A place to write to the message queue
        msg_queue_tx_acm:
            embassy_sync::channel::Sender<'static, CriticalSectionRawMutex, Message, MSG_QUEUE_LEN>,
        leds: bsp::Leds,
        usb_dev: embassy_usb::UsbDevice<'static, hal::usb::Driver<'static, HardwareVbusDetect>>,
        usb_acm_write_adapter: WriteAsyncPipeAdapter,
        usb_acm_reader: embassy_sync::pipe::Reader<'static, CriticalSectionRawMutex, ACM_QUEUE_LEN>,
        usb_acm: embassy_usb::class::cdc_acm::CdcAcmClass<
            'static,
            hal::usb::Driver<'static, HardwareVbusDetect>,
        >,
    }

    #[shared]
    struct MySharedResources {}

    #[init]
    fn init(ctx: init::Context) -> (MySharedResources, MyLocalResources) {
        let board = bsp::init().unwrap();
        Mono::start(ctx.core.SYST, 64_000_000);

        // Create the driver, from the HAL.
        let driver = hal::usb::Driver::new(board.usbd, Irqs, HardwareVbusDetect::new(Irqs));

        // Create embassy-usb Config
        let mut config =
            embassy_usb::Config::new(consts::USB_VID_DEMO, consts::USB_PID_DONGLE_PUZZLE);
        config.manufacturer = Some("Ferrous Systems");
        config.product = Some("Dongle Loopback");
        config.max_packet_size_0 = MAX_ACM_PACKET_SIZE as u8;

        static STATE: StaticCell<embassy_usb::class::cdc_acm::State> = StaticCell::new();
        let state = STATE.init(embassy_usb::class::cdc_acm::State::new());

        // Create embassy-usb DeviceBuilder using the driver and config.
        // It needs some buffers for building the descriptors.
        static CONFIG_DESC: StaticCell<[u8; 256]> = StaticCell::new();
        static BOS_DESC: StaticCell<[u8; 256]> = StaticCell::new();
        static MSOS_DESC: StaticCell<[u8; 128]> = StaticCell::new();
        static CONTROL_BUF: StaticCell<[u8; 128]> = StaticCell::new();

        let mut builder = embassy_usb::Builder::new(
            driver,
            config,
            &mut CONFIG_DESC.init([0; 256])[..],
            &mut BOS_DESC.init([0; 256])[..],
            &mut MSOS_DESC.init([0; 128])[..],
            &mut CONTROL_BUF.init([0; 128])[..],
        );

        // Create classes on the builder.
        let usb_acm = cdc_acm::CdcAcmClass::new(&mut builder, state, MAX_ACM_PACKET_SIZE as u16);

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

        static STATE_HID: StaticCell<embassy_usb::class::hid::State> = StaticCell::new();
        let state_hid = STATE_HID.init(embassy_usb::class::hid::State::new());
        // Create classes on the builder.
        let config = embassy_usb::class::hid::Config {
            report_descriptor: desc,
            request_handler: None,
            poll_ms: 200,
            max_packet_size: 64,
        };

        let hid_rw = hid::HidReaderWriter::<_, 3, 3>::new(&mut builder, state_hid, config);
        // We are only interested in reading from the HID interface.
        let (hid_reader, _) = hid_rw.split();

        // Build the builder.
        let usb_dev = builder.build();

        let current_channel: u8 = 25;
        defmt::debug!("Configuring radio...");
        #[cfg(feature = "dk")]
        let radio = {
            let mut radio = hal::radio::ieee802154::Radio::new(
                unsafe { hal::Peripherals::steal() }.RADIO,
                Irqs,
            );

            // set TX power to its maximum value
            radio.set_transmission_power(8);
            radio.set_channel(current_channel);
            defmt::debug!(
                "Radio initialized and configured with TX power set to the maximum value"
            );
            radio
        };
        #[cfg(not(feature = "dk"))]
        let mut radio = board.radio;
        #[cfg(not(feature = "dk"))]
        radio.set_channel(current_channel);

        static MSG_QUEUE: static_cell::ConstStaticCell<
            embassy_sync::channel::Channel<CriticalSectionRawMutex, Message, MSG_QUEUE_LEN>,
        > = static_cell::ConstStaticCell::new(embassy_sync::channel::Channel::new());
        static ACM_QUEUE: static_cell::ConstStaticCell<
            embassy_sync::pipe::Pipe<CriticalSectionRawMutex, ACM_QUEUE_LEN>,
        > = static_cell::ConstStaticCell::new(embassy_sync::pipe::Pipe::new());

        let msg_queue = MSG_QUEUE.take();
        let (acm_reader, acm_writer) = ACM_QUEUE.take().split();

        let acm_adapter = WriteAsyncPipeAdapter {
            buffer: heapless::String::<256>::new(),
            usb_acm_writer: acm_writer,
        };

        defmt::debug!("Building structures...");
        let shared = MySharedResources {};
        let local = MyLocalResources {
            radio,
            current_channel,
            packet: bsp::hal::radio::ieee802154::Packet::new(),
            timer: board.timer,
            rx_count: 0,
            err_count: 0,
            msg_queue_rx: msg_queue.receiver(),
            msg_queue_tx_acm: msg_queue.sender(),
            leds: board.leds,
            usb_dev,
            usb_acm,
            usb_acm_write_adapter: acm_adapter,
            usb_acm_reader: acm_reader,
        };

        usb_dev::spawn().unwrap();
        usb_acm::spawn().unwrap();
        let _ = usb_hid::spawn(hid_reader, msg_queue.sender());
        radio::spawn().unwrap();

        defmt::debug!("Init Complete!");

        // Set the ARM SLEEPONEXIT bit to go to sleep after handling interrupts
        // See https://developer.arm.com/docs/100737/0100/power-management/sleep-mode/sleep-on-exit-bit
        // TODO: Unfortunately, this does not work yet. Radio packets are not
        // arriving properly.
        //ctx.core.SCB.set_sleepdeep();

        (shared, local)
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            // Now Wait For Interrupt is used instead of a busy-wait loop
            // to allow MCU to sleep between interrupts
            // https://developer.arm.com/documentation/ddi0406/c/Application-Level-Architecture/Instruction-Details/Alphabetical-list-of-instructions/WFI
            //
            // TODO: Unfortunately, this does not work yet. Radio packets are not
            // arriving properly.
            // rtic::export::wfi()
            cortex_m::asm::nop();
        }
    }

    #[task(local = [usb_dev], priority = 1)]
    async fn usb_dev(ctx: usb_dev::Context) {
        ctx.local.usb_dev.run().await;
    }

    #[task(priority = 1)]
    async fn usb_hid(
        _ctx: usb_hid::Context,
        // Need to send this by value, because it is consumed by the run method.
        usb_hid_reader: hid::HidReader<'static, hal::usb::Driver<'static, HardwareVbusDetect>, 3>,
        msg_queue_tx_hid: embassy_sync::channel::Sender<
            'static,
            CriticalSectionRawMutex,
            Message,
            MSG_QUEUE_LEN,
        >,
    ) {
        let mut req_handler = HidTransferHandler(msg_queue_tx_hid);
        usb_hid_reader.run(false, &mut req_handler).await;
    }

    #[task(local = [usb_acm, msg_queue_tx_acm, usb_acm_reader], priority = 1)]
    async fn usb_acm(mut ctx: usb_acm::Context) {
        loop {
            // Wait for up to 200 ms for a connection, discard ACM data otherwise.
            match Mono::timeout_after(200.millis(), ctx.local.usb_acm.wait_connection()).await {
                Ok(_) => {
                    connected_usb_acm(&mut ctx).await;
                }
                Err(_) => {
                    let mut dummy_buf: [u8; 32] = [0; 32];
                    // Timeout. Consume the message queue.
                    while let Ok(_bytes_read) = ctx.local.usb_acm_reader.try_read(&mut dummy_buf) {}
                }
            }
        }
    }

    async fn connected_usb_acm(ctx: &mut usb_acm::Context<'_>) {
        let mut buffer = [0u8; MAX_ACM_PACKET_SIZE];
        loop {
            // Poll for a frame for up to 100 milliseconds.
            if let Ok(result) =
                Mono::timeout_after(100.millis(), ctx.local.usb_acm.read_packet(&mut buffer)).await
            {
                match result {
                    Ok(n) => {
                        if n > 0 {
                            for b in &buffer[0..n] {
                                if *b == b'?' {
                                    // User pressed "?" in the terminal
                                    _ = ctx.local.msg_queue_tx_acm.send(Message::WantInfo).await;
                                }
                            }
                        }
                    }
                    Err(e) => match e {
                        embassy_usb::driver::EndpointError::BufferOverflow => {
                            panic!("unexpected buffer overflow")
                        }
                        embassy_usb::driver::EndpointError::Disabled => (),
                    },
                }
            }
            while let Ok(bytes_read) = ctx
                .local
                .usb_acm_reader
                .try_read(&mut buffer[0..MAX_ACM_PACKET_SIZE - 1])
            {
                match Mono::timeout_after(
                    20.millis(),
                    ctx.local.usb_acm.write_packet(&buffer[0..bytes_read]),
                )
                .await
                {
                    Ok(result) => {
                        if let Err(e) = result {
                            match e {
                                embassy_usb::driver::EndpointError::BufferOverflow => {
                                    panic!("unexpected buffer overflow")
                                }
                                embassy_usb::driver::EndpointError::Disabled => (),
                            }
                        }
                    }
                    Err(_e) => {
                        // Timeout. Consume the message queue.
                        while let Ok(_bytes_read) = ctx.local.usb_acm_reader.try_read(&mut buffer) {
                        }
                    }
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

    #[task(local = [
        radio,
        current_channel,
        packet,
        timer,
        rx_count,
        err_count,
        msg_queue_rx,
        leds,
        usb_acm_write_adapter,
    ], priority = 2)]
    async fn radio(mut ctx: radio::Context) {
        defmt::info!(
            "deviceid={=u32:08x}{=u32:08x} channel={=u8} TxPower=+8dBm app=loopback-fw",
            bsp::deviceid1(),
            bsp::deviceid0(),
            ctx.local.current_channel
        );

        #[cfg(not(feature = "dk"))]
        ctx.local.leds.ld1_green.on();
        #[cfg(not(feature = "dk"))]
        ctx.local.leds.ld2_rgb.blue_only();

        let mut dict: heapless::LinearMap<u8, u8, 128> = heapless::LinearMap::new();
        for (&plain, &cipher) in PLAIN_LETTERS.iter().zip(CIPHER_LETTERS.iter()) {
            let _ = dict.insert(plain, cipher);
        }

        loop {
            while let Ok(msg) = ctx.local.msg_queue_rx.try_receive() {
                match msg {
                    Message::WantInfo => {
                        defmt::info!(
                            "rx={=u32}, err={=u32}, ch={=u8}, app=puzzle-fw",
                            ctx.local.rx_count,
                            ctx.local.err_count,
                            ctx.local.current_channel
                        );
                        let _ = writeln!(
                            &mut ctx.local.usb_acm_write_adapter,
                            "\nrx={}, err={}, ch={}, app=puzzle-fw",
                            ctx.local.rx_count, ctx.local.err_count, ctx.local.current_channel
                        );
                        ctx.local.usb_acm_write_adapter.flush().await;
                    }
                    Message::ChangeChannel(n) => {
                        defmt::info!("Changing Channel to {}", n);
                        let _ = writeln!(
                            &mut ctx.local.usb_acm_write_adapter,
                            "\nChanging Channel to {}",
                            n
                        );
                        ctx.local.usb_acm_write_adapter.flush().await;

                        if !(11..=26).contains(&n) {
                            defmt::info!("Bad Channel {}!", n);
                        } else {
                            ctx.local.radio.set_channel(n);
                        }
                    }
                }
            }

            defmt::debug!("Waiting for packet..");

            // Poll for a frame for up to 200 milliseconds.
            if let Ok(result) =
                Mono::timeout_after(200.millis(), ctx.local.radio.receive(ctx.local.packet)).await
            {
                match result {
                    Ok(_) => {
                        #[cfg(not(feature = "dk"))]
                        ctx.local.leds.ld1_green.toggle();
                        defmt::info!(
                            "Received {=u8} bytes (LQI={})",
                            ctx.local.packet.len(),
                            ctx.local.packet.lqi(),
                        );
                        *ctx.local.rx_count += 1;

                        let mut reply = true;
                        match handle_packet(ctx.local.packet, &dict) {
                            None => {
                                // not enough bytes - send nothing back
                                reply = false;
                            }
                            Some(Command::SendSecret) => {
                                ctx.local
                                    .packet
                                    .set_len(ENCODED_MESSAGE.len() as u8 + ADDR_BYTES as u8);
                                for (src, dest) in ENCODED_MESSAGE
                                    .iter()
                                    .zip(&mut ctx.local.packet[ADDR_BYTES..])
                                {
                                    *dest = *src;
                                }

                                let _ = writeln!(&mut ctx.local.usb_acm_write_adapter, "TX Secret");
                                ctx.local.leds.ld2_rgb.blue.on();
                                ctx.local.leds.ld2_rgb.green.off();
                                ctx.local.leds.ld2_rgb.red.off();
                            }
                            Some(Command::MapChar(plain, cipher)) => {
                                ctx.local.packet.set_len(1 + ADDR_BYTES as u8);
                                ctx.local.packet[ADDR_BYTES] = cipher;
                                let _ = writeln!(
                                    &mut ctx.local.usb_acm_write_adapter,
                                    "TX Map({plain}) => {cipher}"
                                );
                                ctx.local.leds.ld2_rgb.blue.off();
                                ctx.local.leds.ld2_rgb.green.on();
                                ctx.local.leds.ld2_rgb.red.off();
                            }
                            Some(Command::Correct) => {
                                let message = b"correct";
                                ctx.local
                                    .packet
                                    .set_len(message.len() as u8 + ADDR_BYTES as u8);
                                for (src, dest) in
                                    message.iter().zip(&mut ctx.local.packet[ADDR_BYTES..])
                                {
                                    *dest = *src;
                                }
                                let _ =
                                    writeln!(&mut ctx.local.usb_acm_write_adapter, "TX Correct");
                                ctx.local.leds.ld2_rgb.blue.on();
                                ctx.local.leds.ld2_rgb.green.on();
                                ctx.local.leds.ld2_rgb.red.on();
                            }
                            Some(Command::Wrong) => {
                                let message = b"incorrect";
                                ctx.local
                                    .packet
                                    .set_len(message.len() as u8 + ADDR_BYTES as u8);
                                for (src, dest) in
                                    message.iter().zip(&mut ctx.local.packet[ADDR_BYTES..])
                                {
                                    *dest = *src;
                                }
                                let _ =
                                    writeln!(&mut ctx.local.usb_acm_write_adapter, "TX Incorrect");
                                ctx.local.leds.ld2_rgb.blue.off();
                                ctx.local.leds.ld2_rgb.green.on();
                                ctx.local.leds.ld2_rgb.red.on();
                            }
                        }
                        // send packet after 500us (we know the client waits for 10ms and
                        // we want to ensure they are definitely in receive mode by the
                        // time we send this reply)
                        if reply {
                            Mono::delay(500.micros()).await;
                            if let Err(e) = ctx.local.radio.try_send(ctx.local.packet).await {
                                ctx.local.usb_acm_write_adapter.flush().await;
                                let _ = writeln!(
                                    &mut ctx.local.usb_acm_write_adapter,
                                    "\nWriting reply packet failed with error {:?}",
                                    e
                                );
                            }
                        }
                        ctx.local.usb_acm_write_adapter.flush().await;
                    }
                    Err(_e) => {
                        defmt::debug!("RX fail!");
                        let _ = ctx
                            .local
                            .usb_acm_write_adapter
                            .usb_acm_writer
                            .write("!".as_bytes())
                            .await;
                        *ctx.local.err_count += 1;
                    }
                }
            }
        }
    }

    fn handle_packet(
        packet: &mut hal::radio::ieee802154::Packet,
        dict: &heapless::LinearMap<u8, u8, 128>,
    ) -> Option<Command> {
        let payload = packet.get_mut(ADDR_BYTES..)?;
        if payload.is_empty() {
            Some(Command::SendSecret)
        } else if payload.len() == 1 {
            // They give us plaintext, we give them ciphertext
            let plain = payload[0];
            let cipher = *dict.get(&plain).unwrap_or(&0);
            Some(Command::MapChar(plain, cipher))
        } else {
            // They give us plaintext, we tell them if it is correct
            // Encrypt every byte of plaintext they give us
            for slot in payload.iter_mut() {
                if let Some(c) = dict.get(slot) {
                    *slot = *c;
                } else {
                    *slot = 0;
                }
            }
            if &payload[..] == ENCODED_MESSAGE {
                Some(Command::Correct)
            } else {
                Some(Command::Wrong)
            }
        }
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // Safety: We never exit from this function and we are single core.
    #[cfg(not(feature = "dk"))]
    let mut red_led = unsafe { RgbLed::steal() };
    #[cfg(not(feature = "dk"))]
    red_led.red_only();
    if let Some(location) = info.location() {
        defmt::error!("Panic at {}:{}", location.file(), location.line());
    } else {
        defmt::error!("Panic at unknown location");
    }
    loop {
        core::hint::spin_loop();
    }
}

defmt::timestamp!("{=u64:tus}", bsp::uptime_us());

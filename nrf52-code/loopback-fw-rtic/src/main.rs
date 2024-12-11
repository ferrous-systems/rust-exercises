//! The Loopback Firmware, written using RTIC
//!
//! Currently this runs on the Developer Kit and not the Dongle, because it's
//! easier to debug.
//!
//! And it doesn't do any radio stuff - just USB Serial.

#![no_main]
#![no_std]

use defmt_rtt as _;

#[rtic::app(device = dk, peripherals = false)]
mod app {
    use core::mem::MaybeUninit;
    use rtic_monotonics::systick::prelude::*;

    systick_monotonic!(Mono, 100);

    #[local]
    struct MyLocalResources {}

    #[shared]
    struct MySharedResources {
        usb_serial: usbd_serial::SerialPort<'static, dk::UsbDevice>,
        usb_device: usb_device::device::UsbDevice<'static, dk::UsbDevice>,
    }

    #[init(local = [
        usb_alloc: MaybeUninit<usb_device::bus::UsbBusAllocator<dk::UsbDevice>> = MaybeUninit::uninit()
    ])]
    fn init(cx: init::Context) -> (MySharedResources, MyLocalResources) {
        let board = dk::init().unwrap();
        Mono::start(cx.core.SYST, 64_000_000);

        defmt::debug!("Building USB allocator...");
        let usb_alloc = cx
            .local
            .usb_alloc
            .write(usb_device::bus::UsbBusAllocator::new(board.usbd));

        defmt::debug!("Creating usb_serial...");
        let usb_serial = usbd_serial::SerialPort::new(usb_alloc);

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

        defmt::debug!("Spawning task...");
        foo::spawn().ok();

        defmt::debug!("Building structures...");
        let shared = MySharedResources {
            usb_serial,
            usb_device,
        };
        let local = MyLocalResources {};

        defmt::debug!("Init Complete!");

        (shared, local)
    }

    #[task(shared = [usb_serial])]
    async fn foo(mut cx: foo::Context) {
        loop {
            defmt::info!("hello from foo");
            let _ = cx.shared.usb_serial.lock(|usb_serial| {
                usb_serial.write(b"Boopity boop");
            });
            Mono::delay(1000.millis()).await;
        }
    }

    /// USB Interrupt Handler
    ///
    /// USB Device is set to fire this whenever there's a Start of Frame from
    /// the USB Host.
    #[task(binds = USBD, shared = [usb_serial, usb_device])]
    fn usb_isr(cx: usb_isr::Context) {
        (cx.shared.usb_serial, cx.shared.usb_device).lock(|usb_serial, usb_device| {
            usb_device.poll(&mut [usb_serial]);
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

defmt::timestamp!("{=u64:tus}", dk::uptime_us());

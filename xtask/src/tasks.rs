use std::{
    io::{self, Write as _},
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use color_eyre::eyre::{anyhow, bail};
use hidapi::HidApi;
use serialport::SerialPortType;

pub fn change_channel(channel: &str) -> color_eyre::Result<()> {
    fn check_pid(pid: u16) -> bool {
        pid == consts::USB_PID_DONGLE_LOOPBACK || pid == consts::USB_PID_DONGLE_PUZZLE
    }

    let api = HidApi::new()?;
    let dev = api
        .device_list()
        .filter(|dev| dev.vendor_id() == consts::USB_VID_DEMO && check_pid(dev.product_id()))
        .next()
        .ok_or_else(|| anyhow!("device not found"))?
        .open_device(&api)?;

    let chan = channel.parse::<u8>()?;
    if chan < 11 || chan > 26 {
        bail!("channel is out of range (`11..=26`)")
    }
    const REPORT_ID: u8 = 0;
    dev.write(&[REPORT_ID, chan])?;
    println!("requested channel change to channel {}", chan);

    Ok(())
}

pub fn serial_term() -> color_eyre::Result<()> {
    let mut once = true;
    let dongle = loop {
        if let Some(dongle) = serialport::available_ports()?
            .into_iter()
            .filter(|info| match &info.port_type {
                SerialPortType::UsbPort(usb) => usb.vid == consts::USB_VID_DEMO,
                _ => false,
            })
            .next()
        {
            break dongle;
        } else if once {
            once = false;

            eprintln!("(waiting for the Dongle to be connected)");
        }
    };

    let mut port = serialport::new(&dongle.port_name, 115200).open()?;
    port.set_timeout(Duration::from_millis(10))?;

    static CONTINUE: AtomicBool = AtomicBool::new(true);

    // properly close the serial device on Ctrl-C
    ctrlc::set_handler(|| CONTINUE.store(false, Ordering::Relaxed))?;

    let stdout = io::stdout();
    while CONTINUE.load(Ordering::Relaxed) {
        let mut read_buf = [0u8; 8];
        match port.read(&mut read_buf) {
            Ok(n) => {
                let mut stdout = stdout.lock();
                stdout.write_all(&read_buf[..n])?;
                stdout.flush()?;
            }
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                // Go around
            }
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
    }

    eprintln!("(closing the serial port)");
    Ok(())
}

/// List all the USB VIDs and PIDs, and highlight any we know about
pub fn usb_list() -> color_eyre::Result<()> {
    for dev in nusb::list_devices()? {
        let suffix = match (dev.vendor_id(), dev.product_id()) {
            (0x1366, pid) if (pid >> 8) == 0x10 || (pid >> 8) == 0x01 => {
                " <- J-Link on the nRF52840 Development Kit"
            }
            (0x1915, 0x521f) => " <- nRF52840 Dongle (in bootloader mode)",
            (consts::USB_VID_DEMO, consts::USB_PID_DONGLE_LOOPBACK) => {
                " <- nRF52840 Dongle (loopback-fw)"
            }
            (consts::USB_VID_DEMO, consts::USB_PID_DONGLE_PUZZLE) => {
                " <- nRF52840 Dongle (puzzle-fw)"
            }
            (consts::USB_VID_DEMO, consts::USB_PID_RTIC_DEMO) => {
                " <- nRF52840 on the nRF52840 Development Kit"
            }
            _ => "",
        };

        println!(
            "Bus {:03} Device {:03}: ID {:04x}:{:04x}{}",
            dev.bus_number(),
            dev.device_address(),
            dev.vendor_id(),
            dev.product_id(),
            suffix
        );
    }

    Ok(())
}

pub fn usb_descriptors() -> color_eyre::Result<()> {
    for dev in nusb::list_devices()? {
        if dev.vendor_id() == consts::USB_VID_DEMO && dev.product_id() == consts::USB_PID_RTIC_DEMO
        {
            println!("Found RTIC demo on Address {}", dev.device_address());
            println!("{:#?}", dev);
            if let Ok(device) = dev.open() {
                for (cfgno, config) in device.configurations().enumerate() {
                    println!("config{}: {:#?}", cfgno, config);
                }
            }
            return Ok(());
        }
    }

    bail!("nRF52840 USB device not found")
}

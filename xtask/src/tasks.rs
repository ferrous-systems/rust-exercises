use std::{
    io::{self, Write as _},
    sync::atomic::{AtomicBool, Ordering},
    thread,
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

    static CONTINUE: AtomicBool = AtomicBool::new(true);

    // properly close the serial device on Ctrl-C
    ctrlc::set_handler(|| CONTINUE.store(false, Ordering::Relaxed))?;

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut read_buf = [0; 64];
    while CONTINUE.load(Ordering::Relaxed) {
        if port.bytes_to_read()? != 0 {
            let n = port.read(&mut read_buf)?;
            stdout.write_all(&read_buf[..n])?;
            stdout.flush()?;
        } else {
            // time span between two consecutive FS USB packets
            thread::sleep(Duration::from_millis(1));
        }
    }

    eprintln!("(closing the serial port)");
    Ok(())
}

pub fn usb_list() -> color_eyre::Result<()> {
    for dev in rusb::devices()?.iter() {
        let desc = dev.device_descriptor()?;
        let suffix = match (desc.vendor_id(), desc.product_id()) {
            (0x1366, pid) if (pid >> 8) == 0x10 || (pid >> 8) == 0x01 => " <- J-Link on the nRF52840 Development Kit",
            (0x1915, 0x521f) => " <- nRF52840 Dongle (in bootloader mode)",
            (consts::USB_VID_DEMO, consts::USB_PID_DONGLE_LOOPBACK) => " <- nRF52840 Dongle (loopback.hex)",
            (consts::USB_VID_DEMO, consts::USB_PID_DONGLE_PUZZLE) => " <- nRF52840 Dongle (puzzle.hex)",
            (consts::USB_VID_DEMO, consts::USB_PID_RTIC_DEMO) => " <- nRF52840 on the nRF52840 Development Kit",
            _ => "",
        };

        println!("{:?}{}", dev, suffix);
    }

    Ok(())
}

pub fn usb_descriptors() -> color_eyre::Result<()> {
    for dev in rusb::devices()?.iter() {
        let dev_desc = dev.device_descriptor()?;
        if dev_desc.vendor_id() == consts::USB_VID_DEMO && dev_desc.product_id() == consts::USB_PID_RTIC_DEMO {
            println!("{:#?}", dev_desc);
            println!("address: {}", dev.address());
            for i in 0..dev_desc.num_configurations() {
                let conf_desc = dev.config_descriptor(i)?;
                println!("config{}: {:#?}", i, conf_desc);

                for iface in conf_desc.interfaces() {
                    println!(
                        "iface{}: {:#?}",
                        iface.number(),
                        iface.descriptors().collect::<Vec<_>>()
                    );
                }
            }

            // TODO open the device; this will force the OS to configure it
            // let mut handle = dev.open()?;

            return Ok(());
        }
    }

    bail!("nRF52840 USB device not found")
}
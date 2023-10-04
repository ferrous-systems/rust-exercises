#![deny(warnings)]

mod tasks;

use std::env;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // first arg is the name of the executable; skip it
    let args = env::args().skip(1).collect::<Vec<_>>();
    let args = args.iter().map(|arg| &arg[..]).collect::<Vec<_>>();

    match &args[..] {
        ["change-channel", channel] => tasks::change_channel(channel),
        ["serial-term"] => tasks::serial_term(),
        ["usb-list"] => tasks::usb_list(),
        _ => {
            eprintln!(
                "cargo xtask
Workshop-specific tools

USAGE:
    cargo xtask [COMMAND]

COMMANDS:
    change-channel [NUMBER]  instructs the nRF Dongle to listen to a different radio channel
    serial-term              displays the log output of the Dongle
    usb-list                 list all connected USB devices; highlights workshop devices
"
            );

            Ok(())
        }
    }
}

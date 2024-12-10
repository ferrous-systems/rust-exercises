#![no_main]
#![no_std]

use core::time::Duration;

use cortex_m_rt::entry;
// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use radio_app as _;

#[entry]
fn main() -> ! {
    // to enable more verbose logs, set the `DEFMT_LOG` environment variable.

    let board = dk::init().unwrap();

    let mut led = board.leds._1;
    let mut timer = board.timer;

    for _ in 0..10 {
        led.toggle();
        timer.wait(Duration::from_secs(1));
        defmt::println!("LED toggled @ {=u64:tus}", dk::uptime_us());
    }

    dk::exit()
}

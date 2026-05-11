#![no_main]
#![no_std]

use core::time::Duration;

use cortex_m_rt::entry;
// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use hal_app as _;

#[entry]
fn main() -> ! {
    // we don't wait for RTT, so that the LED will blink even without probe-rs
    let board = dk::init_with(dk::InitOptions { wait_for_rtt: false }).unwrap();

    let mut led = board.leds._1;
    let mut timer = board.timer;

    for _ in 0..10 {
        led.toggle();
        timer.wait(Duration::from_secs(1));
        defmt::debug!("LED toggled @ {=u64:tus}", dk::uptime_us());
    }

    dk::exit()
}

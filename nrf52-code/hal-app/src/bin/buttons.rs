#![no_main]
#![no_std]

use core::time::Duration;

use cortex_m_rt::entry;
// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use hal_app as _;

#[entry]
fn main() -> ! {
    let board = dk::init().unwrap();

    let mut led = board.leds._1;
    let mut timer = board.timer;
    // Uncomment the line below
    // 👇
    // let mut button = board.buttons._1;

    defmt::println!("Polling button every 100ms");
    loop {
        // Replace `true` with `button.is_pressed()`
        // 👇
        if true {
            led.on();
        } else {
            led.off();
        }
        timer.wait(Duration::from_millis(100));
    }
}

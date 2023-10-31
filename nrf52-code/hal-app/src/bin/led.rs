#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use hal_app as _;

#[entry]
fn main() -> ! {
    // to enable more verbose logs, go to your `Cargo.toml` and set defmt logging levels
    // to `defmt-trace` by changing the `default = []` entry in `[features]`

    let board = dk::init().unwrap();

    let mut leds = board.leds;
    leds._1.on();
    leds._2.off();
    leds._3.off();
    leds._4.on();

    // this program does not `exit`; use Ctrl+C to terminate it
    loop {
        asm::nop();
    }
}

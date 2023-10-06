#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use radio_app as _;

#[entry]
fn main() -> ! {
    dk::init().unwrap();

    foo();

    loop {
        asm::bkpt();
    }
}

#[inline(never)]
fn foo() {
    asm::nop();
    bar();
}

#[inline(never)]
fn bar() {
    let i = index();
    let array = [0, 1, 2];
    let x = array[i]; // out of bounds access

    defmt::println!("{}", x);
}

fn index() -> usize {
    3
}

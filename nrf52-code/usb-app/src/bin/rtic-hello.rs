#![no_main]
#![no_std]

// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use usb_app as _;

#[rtic::app(device = dk, peripherals = false)]
mod app {
    use cortex_m::asm;

    #[local]
    struct MyLocalResources {}

    #[shared]
    struct MySharedResources {}

    #[init]
    fn init(_cx: init::Context) -> (MySharedResources, MyLocalResources) {
        dk::init().unwrap();

        defmt::println!("Hello");
        (MySharedResources {}, MyLocalResources {})
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        defmt::println!("world!");

        loop {
            asm::bkpt();
        }
    }
}

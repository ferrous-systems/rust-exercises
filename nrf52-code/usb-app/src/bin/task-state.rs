#![no_main]
#![no_std]

// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use usb_app as _;

#[rtic::app(device = dk, peripherals = false)]
mod app {
    use cortex_m::asm;
    use dk::hal::pac::power::Power;

    #[local]
    struct MyLocalResources {
        power: Power,
    }

    #[shared]
    struct MySharedResources {}

    #[init]
    fn init(_cx: init::Context) -> (MySharedResources, MyLocalResources) {
        let board = dk::init().unwrap();

        let power = board.power;

        power.intenset().write(|w| w.set_usbdetected(true));

        defmt::println!("USBDETECTED interrupt enabled");

        (MySharedResources {}, MyLocalResources { power })
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            defmt::println!("idle: going to sleep");
            asm::wfi();
            defmt::println!("idle: woke up");
        }
    }

    #[task(binds = CLOCK_POWER, local = [power])]
    //                                      ^^^^^^^ resource access list
    fn on_power_event(cx: on_power_event::Context) {
        defmt::println!("POWER event occurred");

        // clear the interrupt flag; otherwise this task will run again after it returns
        cx.local.power.events_usbdetected().write_value(0);
    }
}

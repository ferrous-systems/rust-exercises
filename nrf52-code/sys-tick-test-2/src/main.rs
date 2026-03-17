#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};

#[rtic::app(device = embassy_nrf, peripherals = true, dispatchers = [EGU0_SWI0, EGU1_SWI1])]
mod app {
    use defmt::info;
    use embassy_nrf::{
        Peri,
        gpio::{Level, Output, OutputDrive},
        peripherals,
    };
    use rtic_monotonics::systick::prelude::*;

    systick_monotonic!(Mono, 1000);

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        info!("Hello World!");
        let p = embassy_nrf::init(Default::default());
        let _ = blink::spawn(p.P0_13);
        Mono::start(ctx.core.SYST, 64_000_000);

        (Shared {}, Local {})
    }

    #[idle]
    fn idle(_ctx: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    #[task(priority = 1)]
    async fn blink(_cx: blink::Context, pin: Peri<'static, peripherals::P0_13>) {
        let mut led = Output::new(pin, Level::Low, OutputDrive::Standard);

        loop {
            info!("off!");
            led.set_high();
            Mono::delay(300.millis()).await;
            info!("on!");
            led.set_low();
            Mono::delay(300.millis()).await;
        }
    }
}

//! # standalone-rtic-blink
//!
//! An RTIC application that blinks LD1

#![no_std]
#![no_main]

use defmt_rtt as _;

#[rtic::app(device = nucleo_u5a5zj_bsp)]
mod app {
    use nucleo_u5a5zj_bsp as bsp;
    use rtic_monotonics::systick::prelude::*;

    /// How long between the LED changing
    const BLINK_PERIOD_MS: u32 = 500;

    // A systick based Monotonic timer
    //
    // Runs at 100 Hz (10ms per tick)
    systick_monotonic!(Mono, 100);

    /// The resources we share across tasks
    #[shared]
    struct MySharedResources {}

    /// The resources we dedicate to individual tasks
    #[local]
    struct MyLocalResources {
        green_ld1: bsp::Led,
    }

    /// Init routine
    ///
    /// Runs once at start-up, with interrupts disabled.
    ///
    /// Must create the `#[local]` and `#[shared]` objects.

    #[init]
    fn init(mut cx: init::Context) -> (MySharedResources, MyLocalResources) {
        let board = bsp::NonSecureBoard::new_with(&mut cx.core, cx.device);

        defmt::info!("In init...");

        Mono::start(cx.core.SYST, bsp::HCLK_HZ);

        blink::spawn().expect("failed to spawn blink");

        let shared = MySharedResources {};
        let local = MyLocalResources {
            green_ld1: board.green_ld1,
        };
        (shared, local)
    }

    /// Blinks an LED
    ///
    /// The Green LED spends [`BLINK_PERIOD_MS`](crate::BLINK_PERIOD_MS) on, then the same again turned off
    #[task(local = [green_ld1])]
    async fn blink(cx: blink::Context) -> ! {
        let mut next_blink = Mono::now();
        loop {
            defmt::info!("blink, on...");
            cx.local.green_ld1.on();
            next_blink += BLINK_PERIOD_MS.millis();
            Mono::delay_until(next_blink).await;

            defmt::info!("blink, off...");
            cx.local.green_ld1.off();
            next_blink += BLINK_PERIOD_MS.millis();
            Mono::delay_until(next_blink).await;
        }
    }
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", info);
    cortex_m::asm::bkpt();
    loop {}
}

defmt::timestamp!("{=u32:tus}", nucleo_u5a5zj_bsp::timestamp());

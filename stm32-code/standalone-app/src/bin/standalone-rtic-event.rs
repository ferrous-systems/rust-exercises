//! # standalone-rtic-event
//!
//! A simple Nonsecure State binary running on the NUCLEO-U5A5ZJ using RTIC.
//!
//! Demonstrates how RTIC can handle interrupts by executing short-lived tasks
//! to handle them.

#![no_std]
#![no_main]

use defmt_rtt as _;

#[rtic::app(device = nucleo_u5a5zj_bsp)]
mod app {
    use nucleo_u5a5zj_bsp as bsp;

    /// The resources we share across tasks
    #[shared]
    struct MySharedResources {}

    /// The resources we dedicate to individual tasks
    #[local]
    struct MyLocalResources {}

    /// Init routine
    ///
    /// Runs once at start-up, with interrupts disabled.
    ///
    /// Must create the `#[local]` and `#[shared]` objects.
    #[init]
    fn init(mut cx: init::Context) -> (MySharedResources, MyLocalResources) {
        let _board = bsp::NonSecureBoard::new_with(&mut cx.core, cx.device);

        defmt::info!("Init!");

        (MySharedResources {}, MyLocalResources {})
    }

    /// Runs when there is nothing to do
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            defmt::info!("Idle!");
            cortex_m::asm::wfi();
        }
    }

    /// Runs when USART1 interrupt is active
    #[task(binds = USART1)]
    fn usart1_handler(_cx: usart1_handler::Context) {
        defmt::info!("USART1 IRQ!");
    }
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", info);
    loop {}
}

defmt::timestamp!("{=u32:tus}", nucleo_u5a5zj_bsp::timestamp());

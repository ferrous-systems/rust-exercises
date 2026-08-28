//! # standalone-rtic-task-state
//!
//! A completed version of the `standalone-rtic-event` exercise.

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
    struct MyLocalResources {
        usart1: bsp::hal::usart::Driver<{ bsp::hal::usart::USART1_NS }>,
    }

    /// Init routine
    ///
    /// Runs once at start-up, with interrupts disabled.
    ///
    /// Must create the `#[local]` and `#[shared]` objects.
    #[init]
    fn init(mut cx: init::Context) -> (MySharedResources, MyLocalResources) {
        let mut board = bsp::NonSecureBoard::new_with(&mut cx.core, cx.device);

        defmt::info!("Init!");

        board.usart1.configure(bsp::APB2_PERIPH_CLK_HZ);
        board.usart1.rx_interrupt_enable(true);

        (
            MySharedResources {},
            MyLocalResources {
                usart1: board.usart1,
            },
        )
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
    #[task(binds = USART1, local = [usart1])]
    fn usart1_handler(cx: usart1_handler::Context) {
        defmt::info!("USART1 IRQ!");
    }
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", info);
    cortex_m::asm::bkpt();
    loop {}
}

defmt::timestamp!("{=u32:tus}", nucleo_u5a5zj_bsp::timestamp());

//! # standalone-rtic-uart-complete
//!
//! A completed version of the *STM32 Handle UART Received Data* exercise.

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
        counter: u32,
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
                counter: 0,
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
    #[task(binds = USART1, local = [usart1, counter])]
    fn usart1_handler(cx: usart1_handler::Context) {
        defmt::info!("USART1 IRQ!");
        if let Some(rx_ch) = cx.local.usart1.rx_char() {
            defmt::info!("< 0x{=u8:02x}", rx_ch);
            if rx_ch == b'x' {
                *cx.local.counter += 1;
                defmt::info!("That's {} x's!", cx.local.counter);
            }
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

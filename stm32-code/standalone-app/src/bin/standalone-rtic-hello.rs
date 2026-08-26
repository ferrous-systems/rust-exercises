//! # standalone-rtic-hello
//!
//! A basic RTIC application

#![no_std]
#![no_main]

use defmt_rtt as _;

#[rtic::app(device = nucleo_u5a5zj_bsp)]
mod app {
    use nucleo_u5a5zj_bsp as bsp;

    #[shared]
    struct MySharedResources {}

    #[local]
    struct MyLocalResources {}

    #[init]
    fn init(mut cx: init::Context) -> (MySharedResources, MyLocalResources) {
        let _board = bsp::NonSecureBoard::new_with(&mut cx.core, cx.device);

        defmt::println!("Hello");

        (MySharedResources {}, MyLocalResources {})
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        defmt::println!("world!");

        loop {}
    }
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", info);
    cortex_m::asm::bkpt();
    loop {}
}

#![no_std]
#![no_main]

use {core::sync::atomic::AtomicU32, defmt_rtt as _, panic_probe as _};

static SYSTICK_CNT: AtomicU32 = AtomicU32::new(0);

#[rtic::app(device = embassy_nrf, peripherals = false, dispatchers = [EGU0_SWI0, EGU1_SWI1])]
mod app {
    use defmt::info;
    use embassy_nrf::gpio::{Level, Output, OutputDrive};

    use crate::now_ms;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local) {
        info!("Hello World!");
        let sysclk = 64_000_000_u32;
        let timer_hz = 1000;
        assert!(
            sysclk.is_multiple_of(timer_hz),
            "timer_hz cannot evenly divide sysclk! Please adjust the timer or sysclk frequency."
        );
        let reload = sysclk / timer_hz - 1;

        assert!(reload <= 0x00ff_ffff);
        assert!(reload > 0);

        ctx.core.SYST.disable_counter();
        ctx.core
            .SYST
            .set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
        ctx.core.SYST.set_reload(reload);
        ctx.core.SYST.enable_interrupt();
        ctx.core.SYST.enable_counter();

        let p = embassy_nrf::init(Default::default());
        let led = Output::new(p.P0_13, Level::Low, OutputDrive::Standard);
        let _ = syst_check::spawn(led);

        (Shared {}, Local {})
    }

    #[idle]
    fn idle(_ctx: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    #[task(priority = 1)]
    async fn syst_check(_cx: syst_check::Context, mut led: Output<'static>) {
        let last_ms = now_ms();
        loop {
            let ms = now_ms();
            if ms > last_ms {
                defmt::info!("SYSTICK {} ms", ms);
                led.toggle();
            }
            embassy_time::Timer::after_millis(200).await;
        }
    }
}
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
unsafe extern "C" fn SysTick() {
    let mut syst = unsafe { cortex_m::Peripherals::steal().SYST };
    if syst.has_wrapped() {
        SYSTICK_CNT.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    }
}

fn now_ms() -> u32 {
    let mut syst = unsafe { cortex_m::Peripherals::steal().SYST };
    if syst.has_wrapped() {
        SYSTICK_CNT.fetch_add(1, core::sync::atomic::Ordering::AcqRel);
    }

    SYSTICK_CNT.load(core::sync::atomic::Ordering::Relaxed)
}

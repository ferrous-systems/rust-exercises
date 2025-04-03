#![no_std]

use cortex_m_rt::exception;

/// Our custom panic handler.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("{}", defmt::Display2Format(info));
    dk::fail();
}

/// The default HardFault handler just spins, so replace it.
#[exception]
unsafe fn HardFault(_ef: &cortex_m_rt::ExceptionFrame) -> ! {
    defmt::error!("HardFault!");
    dk::fail();
}

// this prevents the panic message being printed *twice* when `defmt::panic!` is invoked
#[defmt::panic_handler]
fn defmt_panic() -> ! {
    dk::fail();
}

defmt::timestamp!("{=u64:tus}", dk::uptime_us());

#![no_std]

use cortex_m_rt::exception;
// use panic_probe as _;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("Oh no! {}", defmt::Debug2Format(info));
    dk::fail();
}

/// The default HardFault handler just spins, so replace it.
///
/// probe-run used to set a hardfault breakpoint but probe-rs doesn't, so make
/// the HardFault handler quit out of probe-rs with a breakpoint.
#[exception]
unsafe fn HardFault(_ef: &cortex_m_rt::ExceptionFrame) -> ! {
    dk::fail();
}

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic!` is invoked
#[defmt::panic_handler]
fn defmt_panic() -> ! {
    dk::fail();
}

# RTIC hello

RTIC, Real-Time Interrupt-driven Concurrency, is a framework for building event-driven, time-sensitive applications.

✅ Open the [`nrf52-code/usb-app/src/bin/rtic-hello.rs`](../../nrf52-code/usb-app/src/bin/rtic-hello.rs) file.

RTIC applications are written in RTIC's Domain Specific Language (DSL). The DSL extends Rust syntax with custom attributes like `#[init]` and `#[idle]`.

RTIC makes a clearer distinction between the application's initialization phase, the `#[init]` function, and the application's main loop or main logic, the `#[idle]` function. The initialization phase runs with interrupts disabled and interrupts are re-enabled before the `idle` function is executed.

`rtic::app` is a procedural macro that generates extra Rust code, in addition to the user's functions. The fully expanded version of the macro can be found in the file `target/rtic-expansion.rs`. This file will contain the expansion of the procedural macro for the last compiled RTIC application.

✅ Build the `rtic-hello` example and look at the generated `rtic-expansion.rs` file.

You can use `rustfmt` on `target/rtic-expansion.rs` to make the generated code easier to read. Among other things, the file should contain the following lines. Note that interrupts are disabled during the execution of the `init` function:

```rust ignore
#[doc(hidden)]
#[no_mangle]
unsafe extern "C" fn main() -> ! {
    rtic::export::interrupt::disable();
    let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal().into();
    #[inline(never)]
    fn __rtic_init_resources<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }
    let mut executors_size = 0;
    extern "C" {
        pub static _stack_start: u32;
        pub static __ebss: u32;
    }
    let stack_start = &_stack_start as *const _ as u32;
    let ebss = &__ebss as *const _ as u32;
    if stack_start > ebss {
        if rtic::export::msp::read() <= ebss {
            panic!("Stack overflow after allocating executors");
        }
    }
    __rtic_init_resources(|| {
        let (shared_resources, local_resources) =
            init(init::Context::new(core.into(), executors_size));
        rtic::export::interrupt::enable();
    });
    idle(idle::Context::new())
}
```

# Bare-Metal Firmware on Cortex-R52 - Writing a UART Driver

We have supplied a small Rust no-std application, which is designed to run
inside a QEMU emulation of an Armv8-R Cortex-R52 system. We build the code using
the `armv8r-none-eabihf` target.

The application lives in
[`./qemu-code/uart-driver`](../../qemu-code/uart-driver/).

The application talks to the outside world through a UART driver. We have
provided two - a working one, and a template one that doesn't work which you
need to fix.

## Task 1 - Get UART TX working

Modify the [template driver](../../qemu-code/uart-driver/src/uart_driver.rs) and
complete the missing code sections as commented. You can peek at the [complete
driver](../../qemu-code/uart-driver/src/uart_driver_solution.rs) if you really
need to!

This will involve reading and writing to the given registers. You have been
given an MMIO handle for the UART peripheral as `self.registers`. This is of
type `MmioUart`, which was automatically generated using the `derive_mmio::Mmio`
derive-macro based on the register definition at the bottom of the file.

You'll want to read the [derive-mmio
documentation](https://docs.rs/derive-mmio/), or run `cargo doc --open` on your
project to see the API available.

## Task 2 - Get UART RX working (Optional)

Continue modifying the UART driver so that you can read data. You'll need to
enable RX in the configuration register, and add an appropriate method
to read a single byte, returning `Option<u8>`. Now modify the main loop to
echo back received characters.

You can look in the [CMSDK UART documentation] to see which bit in the `status`
register indicates that the 1-byte long RX FIFO has data in it.

[CMSDK UART documentation]: (https://developer.arm.com/documentation/ddi0479/b/APB-Components/UART/Programmers-model)

## Task 3 - Make the UART a global

Creating a UART on the stack is fine, but we would now like to use our UART from
anywhere in our program.

To do that, we need to do a few things:

### Step 1

Create a static variable, like `static UART: T = something()`. Work out how to initialise that variable.

<details>
<summary>Answer</summary>

```rust ignore
static UART: Uart = unsafe { Uart::new_uart0() };
```

</details>

### Step 2

Use the `critical_section::Mutex` type to solve the `Sync` error that appears.

<details>
<summary>Answer</summary>

```rust ignore
static UART: critical_section::Mutex<Uart> = critical_section::Mutex::new(unsafe { Uart::new_uart0() });

// Now every time you touch the UART you must lock the Mutex first
critical_section::with(|cs| {
   UART.borrow(cs).enable(115200, PERIPHERAL_CLOCK);
});
```

</details>

### Step 3

That's not enough. You only have `&self` to the `Uart` and you need `&mut self`. Try using a `RefCell`!

<details>
<summary>Answer</summary>

```rust ignore
use core::cell::RefCell;

static UART: critical_section::Mutex<RefCell<Uart>> = critical_section::Mutex::new(RefCell::new(unsafe { Uart::new_uart0() }));

// Now every time you touch the UART you must lock the Mutex and borrow the RefCell first
critical_section::with(|cs| {
   UART.borrow_ref_mut(cs).enable(115200, PERIPHERAL_CLOCK);
});
critical_section::with(|cs| {
   let mut uart = UART.borrow_ref_mut(cs);
   _ = writeln!(uart, "Hello, this is Rust!");
});
```

</details>

### Step 4

That's a lot of locking. Make a wrapper type called `GlobalUart` which hides this mess. Implement `core::fmt::Write` for it so can just `writeln!(&UART, "Hello")`.

<details>
<summary>Answer</summary>

```rust ignore
struct GlobalUart {
    inner: critical_section::Mutex<RefCell<Uart>>
}

impl GlobalUart {
    const fn new() -> GlobalUart {
        GlobalUart { inner:  critical_section::Mutex::new(RefCell::new(unsafe { Uart::new_uart0() })) }
    }

    fn enable(&self, baudrate: u32, periph_clk: u32) {
        critical_section::with(|cs| {
            let mut uart = self.inner.borrow_ref_mut(cs);
            uart.enable(baudrate, periph_clk);
        });
    }
}

static UART: GlobalUart = GlobalUart::new();

// Note that we are implementing the trait for reference-to-GlobalUart because
// we don't have mutable access to our static variable.
impl core::fmt::Write for &GlobalUart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            critical_section::with(|cs| {
                let mut uart = self.inner.borrow_ref_mut(cs);
                uart.write(b);
            });
        }
        Ok(())
    }
}

// Note we are writing into a reference-to-GlobalUart
writeln!(&UART, "Hello, this is Rust!")?;
```

</details>

## Running the code

You will need QEMU 9 installed and in your `$PATH` for `cargo run` to work. This
was the first version with Arm Cortex-R52 emulation.

Running the project gives:

```console
$ cargo run
   Compiling uart-exercise v0.1.0 (/Users/jonathan/Documents/ferrous-systems/rust-exercises/qemu-code/uart-driver)
warning: field `registers` is never read
 --> src/uart_driver.rs:9:5
  |
8 | pub struct Uart {
  |            ---- field in this struct
9 |     registers: MmioRegisters,
  |     ^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `uart-exercise` (lib) generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.87s
     Running `qemu-system-arm -machine mps3-an536 -cpu cortex-r52 -semihosting -nographic -kernel target/armv8r-none-eabihf/debug/uart-exercise`
PANIC: PanicInfo { message: I am a panic, location: Location { file: "src/main.rs", line: 45, col: 5 }, can_unwind: true, force_no_backtrace: false }
```

No UART output (the panic comes on semihosting). But also no-one is using that `registers` field. You should fix that.

With the Task 1 completed (or using the solution file) you will get:

```console
$ cargo run
   Compiling uart-exercise v0.1.0 (/Users/jonathan/Documents/ferrous-systems/rust-exercises/qemu-code/uart-driver)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `qemu-system-arm -machine mps3-an536 -cpu cortex-r52 -semihosting -nographic -kernel target/armv8r-none-eabihf/debug/uart-exercise`
Hello, this is Rust!
    1.00     2.00     3.00     4.00     5.00     6.00     7.00     8.00     9.00    10.00
    2.00     4.00     6.00     8.00    10.00    12.00    14.00    16.00    18.00    20.00
    3.00     6.00     9.00    12.00    15.00    18.00    21.00    24.00    27.00    30.00
    4.00     8.00    12.00    16.00    20.00    24.00    28.00    32.00    36.00    40.00
    5.00    10.00    15.00    20.00    25.00    30.00    35.00    40.00    45.00    50.00
    6.00    12.00    18.00    24.00    30.00    36.00    42.00    48.00    54.00    60.00
    7.00    14.00    21.00    28.00    35.00    42.00    49.00    56.00    63.00    70.00
    8.00    16.00    24.00    32.00    40.00    48.00    56.00    64.00    72.00    80.00
    9.00    18.00    27.00    36.00    45.00    54.00    63.00    72.00    81.00    90.00
   10.00    20.00    30.00    40.00    50.00    60.00    70.00    80.00    90.00   100.00
PANIC: PanicInfo { payload: Any { .. }, message: Some(I am a panic), location: Location { file: "src/main.rs", line: 43, col: 5 }, can_unwind: true, force_no_backtrace: false }
```

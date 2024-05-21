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
given the base-address of the UART peripheral as a const generic, and you have
been given constants for the offset of each register from the base address
(assuming you are working with a `*mut u32`).

You'll want to write a private method to read/write each register, and use
[`write_volatile`](https://doc.rust-lang.org/stable/core/ptr/fn.write_volatile.html)
and
[`read_volatile`](https://doc.rust-lang.org/stable/core/ptr/fn.read_volatile.html)
to access them.

## Task 2 - Get UART RX working

Continue modifying the UART driver so that you can read data. You'll need to
enable the RX bit in the configuration register, and add an appropriate method
to read a single byte, returning `Option<u8>`. Now modify the main loop to
echo back received characters.

You'll need to look in the Cortex-M SDK UART
[documentation](https://developer.arm.com/documentation/ddi0479/b/APB-Components/UART/Programmers-model)
to see which bit in the `status` register indicates that the 1-byte long RX FIFO
has data in it.

## Running the code

You will need QEMU 9 installed and in your `$PATH` for `cargo run` to work. This
was the first version with Arm Cortex-R52 emulation.

With the template unfinished:

```console
$ cargo run
   Compiling uart-exercise v0.1.0 (/Users/jonathan/Documents/ferrous-systems/rust-exercises/qemu-code/uart-driver)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `qemu-system-arm -machine mps3-an536 -cpu cortex-r52 -semihosting -nographic -kernel target/armv8r-none-eabihf/debug/uart-exercise`
PANIC: PanicInfo { payload: Any { .. }, message: Some(I am a panic), location: Location { file: "src/main.rs", line: 43, col: 5 }, can_unwind: true, force_no_backtrace: false }
```

With the Task 1 completed:

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

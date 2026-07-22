# Offering Secure APIs

Now our Nonsecure Mode code is running, we we would like to take the next step - to
have it take advantage of secure services offered by the `secure-loader`
program.

Obviously code in Nonsecure Mode cannot just randomly start executing code that
happens to exist in secure region of flash - that would be bad.

```rust,ignore
fn nonsecure_main() {
    // assume we looked at the disassembly and guessed this memory address:
    let secure_read_key: fn() -> u32 = unsafe { core::mem::transmute(0x3001_2480_u32) };

    let key = secure_read_key();

    // oh no
}
```

The rule in TrustZone-M is that when Nonsecure Mode makes a call to a secure
API, that API must:

* begin with an `sg` (Secure Gateway) instruction, and
* exist in a region of memory marked as *Nonsecure Callable* (that is, Nonsecure
  Mode can execute it, but cannot read or write it).

In addition, it's very important that registers are cleaned up when the secure
function returns back to Nonsecure Mode, and that we do not use any data types
that contain padding between fields - because the padding could inadvertently
contain secret information.

We therefore have three steps:

1. Write the secure API function inside `secure-loader`
2. Mark that function as being *Nonsecure callable* (so the compiler will add
   the `sg` entry point, clean up registers on the way out, and tell us off if
   we use inappropriate types with padding)
3. Import that function into the `nonsecure-app` so we can call it.

### Task 1 - Write a Secure API function

We're just going to add a normal, public, `extern "C"` function to our
`secure-loader`. You can do whatever you like here, but our suggestion is:

* Accept a single numeric argument, of type `u32`
* Print the number using semihosting
* If the number is 0, turn the blue LED off, otherwise turn the blue LED on

You *could* accept a pointer to some nonsecure data (like a string to print),
but then we'd have to check the Nonsecure Mode code wasn't trying to trick
Secure Mode by giving it a pointer to secure SRAM (e.g. a pointer the secret
encryption key). That's possible, but let's stick with plain integers for now.

Note that the Blue LED driver is stored as a global `static` variable, inside a
critical-section Mutex. You can observe what the Secure Fault handler does with
the Red LED as an example of how to use that API.

You should mark the function as `#[unsafe(no_mangle)]` so that the entry in the
symbol table is exactly the function name as given, an not something mangled
like `_RNvCskRV3PiDhb8i_22secure_loader_complete19secure_set_blue_led`.

For now the function won't be callable from Nonsecure world, but we'll fix that
next.

<details>
<summary>Solution</summary>

```rust,ignore
/// Control the blue LED
#[unsafe(no_mangle)]
pub extern "C" fn secure_set_blue_led(value: u32) {
    cortex_m_semihosting::hprintln!("secure_set_blue_led({})", value);
    critical_section::with(|cs| {
        if let Some(blue_led) = BLUE_LED.borrow_ref_mut(cs).as_mut() {
            if value == 0 {
                blue_led.off();
            } else {
                blue_led.on();
            }
        }
    });
}
```

</details>

### Task 2 - Export the function

Now we will export the function from Secure Mode. To do this, we will use a new
Rust ABI called `"cmse-nonsecure-entry"`. This is currently unstable, and only
available in nightly releases of Rust, and even then only if you opt-in to the
feature.

You're already using nightly Rust (see the `rust-toolchain.toml` file we gave
you), so you can add this to the top of the `secure-loader.rs` file:

```rust,ignore
#![feature(cmse_nonsecure_entry)]
```

We can then change the ABI of the function:

```rust,ignore
#[unsafe(no_mangle)]
pub extern "cmse-nonsecure-entry" fn secure_set_blue_led(value: u32) {
    // ... as before
}
```

If we build the program and look at the symbol table, we'll see we now have two
functions:

```console
$ cargo build --bin secure-loader
$ rust-objdump -t ./target/thumbv8m.main-none-eabi/debug/secure-loader-complete | grep blue_led
0c1f0000 g     F .gnu.sgstubs    00000008 secure_set_blue_led
0c000566 g     F .text           00000000 __acle_se_secure_set_blue_led
```

(If you don't have `rust-objdump`, you can `cargo install cargo-binutils` to
install it).

The function `__acle_se_secure_set_blue_led` is the *actual* function we just
wrote, and `secure_set_blue_led` is just the secure gateway stub. It lives in a
different section (`.gnu.sgstubs`), meaning we can tell the linker to put that
function into a special region of memory which we are going to mark as
*Nonsecure Callable*, well away from all the secure code.

Let's check the disassembly, just to verify that:

```console
$ rust-objdump -d -j .gnu.sgstubs ./target/thumbv8m.main-none-eabi/debug/secure-loader

./target/thumbv8m.main-none-eabi/debug/secure-loader:	file format elf32-littlearm

Disassembly of section .gnu.sgstubs:

0c1f0000 <secure_set_blue_led>:
 c1f0000: e97f e97f    	sg
 c1f0004: f610 baaf    	b.w	0xc000566 <__acle_se_secure_set_blue_led> @ imm = #-0x1efaa2
 c1f0008: d4d4         	bmi	0xc1effb4 <__erodata+0x1ecc60> @ imm = #-0x58
 c1f000a: d4d4         	bmi	0xc1effb6 <__erodata+0x1ecc62> @ imm = #-0x58
 c1f000c: d4d4         	bmi	0xc1effb8 <__erodata+0x1ecc64> @ imm = #-0x58
 c1f000e: d4d4         	bmi	0xc1effba <__erodata+0x1ecc66> @ imm = #-0x58
 c1f0010: d4d4         	bmi	0xc1effbc <__erodata+0x1ecc68> @ imm = #-0x58
 c1f0012: d4d4         	bmi	0xc1effbe <__erodata+0x1ecc6a> @ imm = #-0x58
 c1f0014: d4d4         	bmi	0xc1effc0 <__erodata+0x1ecc6c> @ imm = #-0x58
 c1f0016: d4d4         	bmi	0xc1effc2 <__erodata+0x1ecc6e> @ imm = #-0x58
 c1f0018: d4d4         	bmi	0xc1effc4 <__erodata+0x1ecc70> @ imm = #-0x58
 c1f001a: d4d4         	bmi	0xc1effc6 <__erodata+0x1ecc72> @ imm = #-0x58
 c1f001c: d4d4         	bmi	0xc1effc8 <__erodata+0x1ecc74> @ imm = #-0x58
 c1f001e: d4d4         	bmi	0xc1effca <__erodata+0x1ecc76> @ imm = #-0x58
 ```

 That looks like an `sg` secure gateway instruction, followed by a branch (`b.w`
 is a wide branch) to the actual function. The rest is padding that we can
 ignore.

 Why is it at `0x0C1F_0000`? Because we told the linker to put it there, using
 the `build.rs` file:

 ```rust
// Use top of Bank 0 for the Secure Gateway stubs
println!("cargo:rustc-link-arg=--section-start=.gnu.sgstubs=0x0C1F0000");
```

Ideally the cortex-m-rt linker script would let us `PROVIDE` a value to the
linker using the `memory.x` linker fragment, but it doesn't, so we crowbar our
way in by telling the linker explicitly where to start that section.

Now we need to mark the memory as Nonsecure callable by adding an entry to the
SAU. We can either hardcode the values, or get them from the linker - let's do
the latter:

```rust,ignore
// These symbols come from the cortex-m-rt linker script
unsafe extern "C" {
    static __veneer_base: u32;
    static __veneer_limit: u32;
}

let nsc_region = SauRegion {
    base_address: (&raw const __veneer_base) as u32,
    limit_address: ((&raw const __veneer_limit) as u32) - 1,
    attribute: SauRegionAttribute::NonSecureCallable,
};

// add this to your slice of SAU regions that you pass to the SAU
```

The term *veneer*, as in a very thin outer layer of material (often wood), is
the name given to the function containing the `sg` and the branch instruction.
The cortex-m-rt linker script *could* have called these `__sg_stubs_start` and
`__sg_stubs_end`, but for whatever reason they did not.

## Task 3 - Call the function

Over on the `nonsecure-app` we can now call this exported function. But where
exactly is it in memory? We know the secure veneers start at `0x0C1F_0000`
because we hard-coded that, but if there are multiple secure gateway functions,
which one starts at which location inside that region? It would be very bad if
we got them mixed up and called the wrong function (with the wrong arguments).

A neat solution to this problem is to ask the linker to export a tiny fragment
of an object file - one with no machine code in it, only a symbol table that
gives a name and address for each exported function.

We did that in the `build.rs` file for `secure-loader`:

```rust
// Emit a file where the symbol table notes where the Secure Gateway stubs are
println!("cargo:rustc-link-arg=--out-implib=target/libsec_bootloader_stubs.o");
```

Over on in the `build.rs` for the `nonsecure-app`, we tell the linker to include
it:

```rust
// Import the file where the symbol table notes where the Secure Gateway stubs are
println!("cargo:rustc-link-arg=target/libsec_bootloader_stubs.o");
// And rebuild our program if it changes
println!("cargo::rerun-if-changed=target/libsec_bootloader_stubs.o");
```

Now, this leads to an interesting problem that `cargo` is not really equipped to
deal with. If we delete the `target` folder - or if we do a fresh checkout - and
then we run `cargo build --bin nonsecure-app`, it will fail because
`target/libsec_bootloader_stubs.o` does not exist. There is unfortunately no way
to tell cargo "hey, before you build binary X, please first build binary Y". The
solution is to use some other tool - a bash script, [a Makefile], [a Justfile],
or something similar.

[a Makefile]: https://www.gnu.org/software/make/
[a Justfile]: https://github.com/casey/just

For now, I was merely careful to direct you to build `secure-loader` first, and
then build `nonsecure-app` later. But now you're editing both you're going to
have to do that yourself.

The following script might help you keep things in order:

```bash
cargo build --bin secure-loader
probe-rs download --chip STM32U5A5ZJ ./target/thumbv8m.main-none-eabi/secure-loader
cargo run --bin nonsecure-app
```

A `probe-rs download` will put the code into flash, but won't execute it or
connect to the log stream. This sequence should ensure your `secure-loader` and
`nonsecure-app` remain in sync.

A production system might also have the `secure-loader` place some API version
number at some well-known place in flash and then have the `nonsecure-app` read
and verify that version before it calls any secure APIs. You could also funnel
everything though a single secure gateway function, and use arguments to the
function to select which operation to perform (a bit like performing a kernel
syscall with `swi`).

For now, simply tell the `nonsecure-app` that the function exists, and call it:

```rust,ignore
unsafe extern "C" {
    safe fn secure_set_blue_led(value: u32);
}

secure_set_blue_led(0);
```

Try adding the secure call to the loop, so that when the Green LED is on, the
Blue LED is off, and vice-versa.

## Try to break out of jail

We're at the end of the exercise, so as a bit of fun, try and get
`nonsecure-app` to read from an illegal part of memory - maybe SRAM1 at
`0x3000_0000` (or its nonsecure alias at `0x2000_0000`). If we set up the SAU
correctly, you should not be able to and you should end up in the SecureFault
handler (which will patiently explain the nature of your attempted crime). You
could also pull put the address of the `__acle_se_secure_set_blue_led` function
and try and call it from `nonsecure-app` directly (you can `transmute` as noted
at the top of this chapter) - that shouldn't work either, because you've skipped
the `sg` entry point.

If you do manage to jailbreak the system, please let me know how!

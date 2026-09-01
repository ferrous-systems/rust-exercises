# TrustZone Intro

## What is TrustZone

Arm processors with TrustZone support two execution states:

* Secure State
* Nonsecure State

The resources of the processor are shared between the two states, but only one
state is executing code at any given moment in time.

Secure state has more permissions than Nonsecure State. That means Secure state
can do things that affect Nonsecure state, but Nonsecure state *cannot* affect
Secure state (except through APIs that you can define and tightly control).

You might, for example, store your encryption keys in an application running in
Secure state, and process your untrusted (and potentially malicious) in
Nonsecure state. It should be **impossible** for any kind of hack on the
Nonsecure code to result in the attacker obtaining the encryption key out of
Secure state.

You can change states by:

* Setting a non-volatile option to control which state the processor starts in.
* Having Secure state call a Nonsecure state entry function, using a special new
  branch instruction.
* Having Nonsecure state call an API provided by Secure state, using a special
  new branch instruction **and** only when branching to a memory address
  starting with a special new *Secure Gateway* marker instruction.

TrustZone is available on both Application-profile Arm processors (Cortex-A,
Cortex-X and Neoverse etc), and on Microcontroller-profile Arm processors
(Cortex-M). Our STM32U5A5ZJ-Q is a Cortex-M33 based device.

## Enable TrustZone

The STM32U5A5ZJ-Q MCU has a small ROM which executes before any user code. This
ROM can be controlled with some non-volatile configuration, known as *Option
Bytes*.

The Option Bytes need to be changed on your board, so that it boots into Secure
State. This is controlled by a bit called `TZEN`.

We have provided a simple program which will set `TZEN=1` in the Option Bytes.
The program is called `step1-option-bytes` and running it will also help us
check we have `probe-rs` installed and that we have permissions to access the
STLinkV3 over USB.

```console
$ cargo run --bin step1-option-bytes
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.03s
     Running `probe-rs run --chip STM32U5A5ZJ target/thumbv8m.main-none-eabi/debug/step1-option-bytes`
      Erasing ✔ 100% [####################]  24.00 KiB @ 180.45 KiB/s (took 0s)
     Finished in 0.95s
Running step1-option-bytes program.
Enable FLASH peripheral...
Unlock FLASH peripheral...
Unlock Option Bytes...
Set Option Bytes...
Program Option Bytes...
Reloading option Bytes. probe-rs is about to crash (and that's OK).

(all the rest of the output is junk from probe-rs that you can ignore)
```

We expect `probe-rs` to crash - setting `TZEN=1` and reloading the Option Bytes
seems to cause the debugger to be disconnected. You can run the program a second
time to check that it worked.

```console
$ cargo run --bin step1-option-bytes
     Running `probe-rs run --chip STM32U5A5ZJ target/thumbv8m.main-none-eabi/debug/step1-option-bytes`
      Erasing ✔ 100% [####################]  24.00 KiB @ 175.37 KiB/s (took 0s)
  Programming ✔ 100% [####################]  20.00 KiB @  28.20 KiB/s (took 1s)
  Finished in 0.95s
Running step1-option-bytes program.
Enable FLASH peripheral...
TZEN=1 already. Doing nothing
```

Press `Ctrl + C` to quit `probe-rs` because the 'step1-option-bytes' program just
enters an infinite loop if it has nothing to do.

## Secure Watermark

Another part of the option bytes controls the "Secure Watermark" - that is,
which pages in Flash are readable from Nonsecure State, and which are reserved
for Secure state usage. We need to adjust the "Secure Watermark" in order for
our example programs work.

We couldn't do that before because you can only do this when your board boots
into Secure state, hence why this binary is called `step2-secure-watermark`.

You can run it like this:

```console
$ cargo run --bin step2-secure-watermark
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.03s
     Running `probe-rs run --chip STM32U5A5ZJ target/thumbv8m.main-none-eabi/debug/step2-secure-watermark`
      Erasing ✔ 100% [####################]   8.00 KiB @  84.80 KiB/s (took 0s)
  Programming ✔ 100% [####################]   6.00 KiB @  23.62 KiB/s (took 0s)
     Finished in 0.45s
Running option-bytes program.
Enable FLASH peripheral...
Unlocking Bank 2 from Secure State. probe-rs is about to crash and that's OK :)

(all the rest of the output is junk from probe-rs that you can ignore)
```

Again, `probe-rs` got upset and disconnected when we changed the option bytes,
and that's still OK. You can run it again to verify that it worked.

```console
$ cargo run --bin step2-secure-watermark
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.02s
     Running `probe-rs run --chip STM32U5A5ZJ target/thumbv8m.main-none-eabi/debug/step2-secure-watermark`
      Erasing ✔ 100% [####################]   8.00 KiB @  84.38 KiB/s (took 0s)
  Programming ✔ 100% [####################]   8.00 KiB @  23.62 KiB/s (took 0s)
     Finished in 0.45s
Running option-bytes program.
Enable FLASH peripheral...
Secure watermark is OK :)
```

Press `Ctrl + C` to quit `probe-rs` because the 'step2-secure-watermark' program just
enters an infinite loop if it has nothing to do.

## Next Steps

Your board is all set up and appears to be working, so are now ready to move on
to the exercises in the following chapters.

If you have any issues, or want to inspect the Option Bytes, you can also use
the official [STM32CubeProgrammer].

[STM32CubeProgrammer]: https://www.st.com/en/development-tools/stm32cubeprog.html

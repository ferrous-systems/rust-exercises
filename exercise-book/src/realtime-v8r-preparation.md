# Bare-Metal Firmware on Cortex-R52 - Preparation

This chapter contains information about the QEMU-based exercises, the required software and an installation guide.

## Required Software

### QEMU, version 9

Available for Windows, macOS or Linux from <https://www.qemu.org/download/>

Note that version 8 or lower will not work. It must be version 9 or higher to support the Cortex-R52.

Ensure that once installed you have `qemu-system-arm` on your path.

### Ferrocene or Rust

If you use Ferrocene, you will need `pre-rolling-2024-05-21` or newer. A
`criticalup.toml` file is provided, you can just `criticalup install` in the
example directory and an appropriate toolchain will be provided.

If you use Rust, you will need a version that supports `armv8r-none-eabihf`.
This should be included in Rust 1.78 or newer, or a nightly from around March
2024 or newer. You will also need to compile the standard library from source -
see the [README](../../qemu-code/uart-driver) for more details.

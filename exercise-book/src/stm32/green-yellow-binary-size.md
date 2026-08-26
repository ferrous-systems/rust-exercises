# Binary Size

ELF files contain metadata like debug information so their size on disk is not a good indication of the amount of Flash the program will use once it's loaded on the target device's memory.

To display the amount of Flash the program will occupy on the target device use the `cargo-size` tool, which is part of the `cargo-binutils` package.

Install the `cargo-binutils` add-on first:

```console
cargo install cargo-binutils
```

✅ Use the following command to print the binary's size:

```console
cargo size --bin standalone-hello -- -A
```

Expected output: The breakdown of the program's static memory usage per *linker section*.

```console
$ cargo size --bin standalone-hello -- -A
   Compiling standalone-app v0.1.0 (/Users/jonathan/Documents/rust-exercises/stm32-code/standalone-app)
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.38s
standalone-hello  :
section                size        addr
.vector_table           600   0x8000000
.text                  9388   0x8000258
.rodata                2164   0x8002704
.data                    56  0x20000000
.gnu.sgstubs              0   0x8002fc0
.bss                     12  0x20000038
.uninit                1024  0x20000044
.defmt                    8         0x0
.debug_loc           202711         0x0
.debug_abbrev         16919         0x0
.debug_info         1218976         0x0
.debug_aranges        39552         0x0
.debug_ranges        106576         0x0
.debug_str          1917769         0x0
.comment                161         0x0
.ARM.attributes          50         0x0
.debug_frame         109592         0x0
.debug_line          381858         0x0
Total               4007416
```

**🔎 More details about each linker section:**

The first three sections are contiguously located in Flash memory -- on the STM32U5A5, flash memory spans from address `0x0800_0000` to `0x0840_0000` (i.e. 4 MiB of flash).

* The `.vector_table` section contains the *vector table*, a data structure required by the Armv8-M specification
* The `.text` section contains the instructions the program will execute
* The `.rodata` section contains constants like strings literals

Skipping `.gnu.sgstubs` (which is empty), the next few sections - `.data`, `.bss` and `.uninit` - are located in RAM. Our RAM spans the address range `0x2000_0000` - `0x200C_0000` (768 KB). These sections contain statically allocated variables (`static` variables), which are either initialised with a value kept in flash, with zero, or with nothing at all.

The remaining sections are debug information, which we ignore for now. But your debugger might refer to them when debugging!

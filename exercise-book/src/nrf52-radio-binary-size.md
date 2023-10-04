# Binary Size

ELF files contain metadata like debug information so their size on disk is not a good indication of the amount of Flash the program will use once it's loaded on the target device's memory.

To display the amount of Flash the program will occupy on the target device use the `cargo-size` tool, which is part of the `cargo-binutils` package.

✅ Use the following command to print the binary's size in system V format.

``` console
cargo size --bin hello -- -A
```

Expected output: The breakdown of the program's static memory usage per *linker section*.

```console
$ cargo size --bin hello -- -A
   Compiling radio v0.0.0 (/Users/jonathan/Documents/rust-exercises/nrf52-exercise-solutions/radio)
    Finished dev [optimized + debuginfo] target(s) in 0.92s
hello  :
section               size        addr
.vector_table          256         0x0
.text                 4992       0x100
.rodata               1108      0x1480
.data                   48  0x2003fbc0
.gnu.sgstubs             0      0x1920
.bss                    12  0x2003fbf0
.uninit               1024  0x2003fbfc
.defmt                   6         0x0
.debug_loc            3822         0x0
.debug_abbrev         3184         0x0
.debug_info         109677         0x0
.debug_aranges        2896         0x0
.debug_ranges         4480         0x0
.debug_str          108868         0x0
.debug_pubnames      40295         0x0
.debug_pubtypes      33582         0x0
.ARM.attributes         56         0x0
.debug_frame          2688         0x0
.debug_line          18098         0x0
.comment                19         0x0
Total               335111
```

**🔎 More details about each linker section:**

The first three sections are contiguously located in Flash memory -- on the nRF52840, flash memory spans from address `0x0000_0000` to `0x0010_0000` (i.e. 1 MiB of flash).

* The `.vector_table` section contains the *vector table*, a data structure required by the Armv7E-M specification
* The `.text` section contains the instructions the program will execute
* The `.rodata` section contains constants like strings literals

Skipping `.gnu.sgstubs` (which is empty), the next few sections - `.data`, `.bss` and `.uninit` - are located in RAM. Our RAM spans the address range `0x2000_0000` - `0x2004_0000` (256 KB). These sections contain statically allocated variables (`static` variables), which are either initialised with a value kept in flash, with zero, or with nothing at all.

The remaining sections are debug information, which we ignore for now. But your debugger might refer to them when debugging!

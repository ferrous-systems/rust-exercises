# nRF52 Code Organization

## Workshop Materials

Clone and change into the [rust-exercises git repository](https://github.com/ferrous-systems/rust-exercises):

```console
git clone https://github.com/ferrous-systems/rust-exercises.git
cd rust-exercises
```

The workshop repository contains all workshop materials, i.e. code snippets, custom tools and the source for this handbook.

## Firmware

The target firmware for the nRF52 for this exercise lives in `./nrf52-code`:

```console
$ tree -L 2
.
├── boards
│   ├── dk
│   └── dongle
├── consts
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
├── radio-app
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
├── radio-app-solutions
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
├── usb-app
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
├── usb-app-solutions
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
├── usb-lib
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
├── usb-lib-solution-get-descriptor-config
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
├── usb-lib-solution-get-device
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
└── usb-lib-solution-set-config
    ├── Cargo.lock
    ├── Cargo.toml
    └── src
28 directories, 16 files
```

### board/dk

Contains a Board Support Package for the nRF52840 Developer Kit.

### board/dongle

Contains precompiled firmware for the nRF52 USB Dongle. Use in the *nRF52 Radio Exercise*.

### consts

Contains constants (e.g. USB Vendor IDs) shared by multiple crates.

### radio

Contains template and solution binary crates for the *nRF Radio* exercise.

### rtic-app

Contains template binary crates for the *nRF USB* exercise.

### rtic-app-solutions

Contains solution binary crates for the *nRF USB* exercise.

### usb-lib

Contains a template library crate for the *nRF USB* exercise. This library can parse USB descriptor information.

### usb-lib-solution-get-descriptor-config

Contains a solution library crate for the *nRF USB* exercise.

### usb-lib-solution-get-device

Contains a solution library crate for the *nRF USB* exercise.

### usb-lib-solution-set-config

Contains a solution library crate for the *nRF USB* exercise.

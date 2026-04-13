# nRF52 Code Organization

## Training Materials

You will need a local copy of the training materials. We recommend the Github release as it contains pre-compiled HTML docs and pre-compiled dongle firmware, but you can clone the repo with `git` and check out the appropriate tag as well if you prefer.

Ask your trainer which release/tag you should be using.

### Github Release

Download the latest release from [the rust-exercises Github release area](https://github.com/ferrous-systems/rust-exercises/releases). Unpack the zip file somewhere you can work on the contents.

### Git checkout

Clone and change into the [rust-exercises git repository](https://github.com/ferrous-systems/rust-exercises):

```console
git clone https://github.com/ferrous-systems/rust-exercises.git
cd rust-exercises
```

The git repository contains all training materials, i.e. code snippets, custom tools and the source for this handbook, but not the pre-compiled dongle firmware.

## Firmware

The target firmware for the nRF52 for this exercise lives in `./nrf52-code`:

```console
$ tree -L 2
.
├── boards
│   ├── dk
│   ├── dk-solution
│   ├── dongle
│   └── dongle-fw
├── consts
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
├── hal-app
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
├── loopback-fw
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
├── puzzle-fw
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── build.rs
│   └── src
├── radio-app
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
│   ├── src
│   └── traces
├── usb-lib
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
└── usb-lib-solutions
    ├── get-descriptor-config
    ├── get-device
    └── set-config

27 directories, 17 files
```

### boards/dk

Contains a Board Support Package for the nRF52840 Developer Kit.

### boards/dk-solution

Contains a Board Support Package for the nRF52840 Developer Kit, with a solution to the [BSP exercise](./nrf52-hal-buttons.md).

### boards/dongle

Contains a Board Support Package for the nRF52840 USB Dongle. You won't be using this.

### boards/dongle-fw

In the release zip file, this contains pre-compiled firmware for the nRF52 USB Dongle, which you use in the *nRF52 Radio Exercise*. In the Git repository, it's empty.

### consts

Contains constants (e.g. USB Vendor IDs) shared by multiple crates.

### hal-app

Contains template and solution binary crates for the *nRF BSP* exercise.

### loopback-fw

Source code for the USB Dongle firmware to implement loopback mode.

### puzzle-fw

Source code for the USB Dongle firmware to implement puzzle mode. No, you won't find the solution to the puzzle in this source directory - nice try!

### radio-app

Contains template and solution binary crates for the *nRF Radio* exercise.

### usb-app

Contains template binary crates for the *nRF USB* exercise.

### usb-app-solutions

Contains solution binary crates for the *nRF USB* exercise.

### usb-lib

Contains a template library crate for the *nRF USB* exercise. This library can parse USB descriptor information.

### usb-lib-solutions/get-descriptor-config

Contains a solution library crate for the *nRF USB* exercise.

### usb-lib-solutions/get-device

Contains a solution library crate for the *nRF USB* exercise.

### usb-lib-solutions/set-config

Contains a solution library crate for the *nRF USB* exercise.

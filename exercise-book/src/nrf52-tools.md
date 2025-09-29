# nRF52 Tools

Follow the relevant section for the operating system that you're using:

* [Linux](#linux)
* [Windows](#windows)
* [macOS](#macos)

Once complete, go to [Setup check](#setup-check).

---

## Linux

<!-- TODO would be nice to have an automated local table of contents here,
     and it's probably not worth putting them in manually. -->

### Install VS Code

Follow the instructions for your distribution on [https://code.visualstudio.com/docs/setup/linux](https://code.visualstudio.com/docs/setup/linux).

### Install dependencies

Some of our tools require a C compiler.

Ensure you have the proper packages installed.
On Debian based distributions you can use:

```console
sudo apt-get install gcc
```

### Configure USB Device access for non-root users

Connect the dongle and check its permissions with these commands:

```console
$ lsusb -d 1915:521f
Bus 001 Device 016: ID 1915:521f Nordic Semiconductor ASA USB Billboard
$ #   ^         ^^

$ # take note of the bus and device numbers that appear for you when run the next command
$ ls -l /dev/bus/usb/001/016
crw-rw-r-- 1 root root 189, 15 May 20 12:00 /dev/bus/usb/001/016
```

The `root root` part in `crw-rw-r-- 1 root root` indicates the device can only be accessed by the `root` user.

To access the USB devices as a non-root user, follow these steps:

1. As root, create `/etc/udev/rules.d/50-ferrous-training.rules` with the following contents:

    ```console
    # udev rules to allow access to USB devices as a non-root user

    # nRF52840 Dongle in bootloader mode
    ATTRS{idVendor}=="1915", ATTRS{idProduct}=="521f", TAG+="uaccess"

    # nRF52840 Dongle applications
    ATTRS{idVendor}=="1209", TAG+="uaccess"

    # nRF52840 Development Kit
    ATTRS{idVendor}=="1366", ENV{ID_MM_DEVICE_IGNORE}="1", TAG+="uaccess"
    ```

2. Run the following command to put the new udev rules into effect

    ```console
    sudo udevadm control --reload-rules
    ```

To check the permissions again, first disconnect and reconnect the dongle. Then run `lsusb`.

```console
$ lsusb
Bus 001 Device 017: ID 1915:521f Nordic Semiconductor ASA 4-Port USB 2.0 Hub

$ ls -l /dev/bus/usb/001/017
crw-rw-r--+ 1 root root 189, 16 May 20 12:11 /dev/bus/usb/001/017
```

The `+` part in `crw-rw-r--+` indicates the device can be accessed without `root` permissions. If you have permission to access them dongle, then the nRF52-DK should also work because both were listed in the udev rules file.

### Install base rust tooling

Go to [https://rustup.rs](https://rustup.rs/) and follow the instructions.

### Install rust analyzer

Open VS Code, find [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) in the marketplace (bottom icon in the left panel), then install it.

### Configure Rust Cross compilation support

Run this command in a terminal:

```console
rustup +stable target add thumbv7em-none-eabihf
```

### Install ELF analysis tools

Run these commands in a terminal:

```console
cargo install cargo-binutils
rustup +stable component add llvm-tools
```

### Third-party tools written in Rust

Install the [`flip-link`](https://crates.io/crates/flip-link), [`nrf-dfu`](https://crates.io/crates/nrfdfu) and [`cyme`](https://crates.io/crates/cyme) tools from source using the following Cargo commands:

```console
cargo install flip-link
cargo install nrfdfu
cargo install cyme
```

Install `probe-rs` 0.29.1 pre-compiled binaries on Linux with:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/download/v0.29.1/probe-rs-tools-installer.sh | sh
```

---

## Windows

### Install VS Code

Go to [https://code.visualstudio.com](https://code.visualstudio.com) and run the installer.

### Associate the device with the WinUSB drivers

On Windows you'll need to associate the nRF52840 Development Kit's USB device to the WinUSB driver.

To do that connect the nRF52840 DK to your PC using micro-USB port J2, then download and run the [Zadig] tool.

[Zadig]: https://zadig.akeo.ie/

In Zadig's graphical user interface,

1. Select the 'List all devices' option from the Options menu at the top.

2. From the device (top) drop down menu select "BULK interface (Interface nnn)"

3. Once that device is selected, `1366 1051` should be displayed in the USB ID field. That's the Vendor ID - Product ID pair.

4. Select 'WinUSB' as the target driver (right side)

5. Click "Install Driver". The process may take a few minutes to complete and might not appear to do anything right away. Click it once and wait.

> You do not need to do anything for the **nRF52840 Dongle** device.

### Install base rust tooling

Go to [https://rustup.rs](https://rustup.rs/) and follow the instructions.

You will need a C compiler to use Rust on Windows. The rustup installer will suggest you install either Visual Studio, or the Build Tools for Visual Studio - either is fine. When that is installing, be sure to select the optional "Desktop development with C++" part of the [C++ build tools package](https://visualstudio.microsoft.com/visual-cpp-build-tools/). The installation may take up to 5.7 GB of disk space. Please also be aware of the license conditions attached to these products, especially in an enterprise environment.


### Install rust analyzer

Open VS Code, find [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) in the marketplace (bottom icon in the left panel), then install it.

If you get a message about `git` not being installed, ignore it!

### Configure Rust Cross compilation support

Run this command in a terminal:

```console
rustup +stable target add thumbv7em-none-eabihf
```

### Install ELF analysis tools

Run these commands in a terminal:

```console
cargo install cargo-binutils
rustup +stable component add llvm-tools
```

### Third-party tools written in Rust

Install the [`flip-link`](https://crates.io/crates/flip-link), [`nrf-dfu`](https://crates.io/crates/nrfdfu) and [`cyme`](https://crates.io/crates/cyme) tools from source using the following Cargo commands:

```console
cargo install flip-link
cargo install nrfdfu
cargo install cyme
```

Install `probe-rs` 0.29.1 pre-compiled binaries on Windows with:

```bash
powershell -c "irm https://github.com/probe-rs/probe-rs/releases/download/v0.29.1/probe-rs-tools-installer.ps1 | iex"
```

---

## macOS

### Install VS Code

Go to [https://code.visualstudio.com](https://code.visualstudio.com) and click on "Download for Mac".

### Install base rust tooling

Go to [https://rustup.rs](https://rustup.rs/) and follow the instructions.

### Install rust analyzer

Open VS Code, find [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) in the marketplace (bottom icon in the left panel), then install it.

### Configure Rust Cross compilation support

Run this command in a terminal:

```console
rustup +stable target add thumbv7em-none-eabihf
```

### Install ELF analysis tools

Run these commands in a terminal:

```console
cargo install cargo-binutils
rustup +stable component add llvm-tools
```

### Third-party tools written in Rust

Install the [`flip-link`](https://crates.io/crates/flip-link), [`nrf-dfu`](https://crates.io/crates/nrfdfu) and [`cyme`](https://crates.io/crates/cyme) tools from source using the following Cargo commands:

```console
cargo install flip-link
cargo install nrfdfu
cargo install cyme
```

Install `probe-rs` 0.29.1 pre-compiled binaries on macOS with:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/download/v0.29.1/probe-rs-tools-installer.sh | sh
```

---

## Setup check

✅ Let's check that you have installed all the tools listed in the previous section.

```console
$ cargo size --version
cargo-size 0.3.6
```

✅ Connect the nRF52840-DK with your computer by plugging the usb-cable into the J2 connector on the DK (the usb connector on the short side of the board).

✅ Use `cyme` to list the USB devices on your computer.

```console
$ cyme
(..)
  2  15  0x1366 0x1051 J-Link                   001050255503      12.0 Mb/s
(..)
```

Your nRF52840-DK should appear as "J-Link" with USB Vendor ID (VID) of 0x1366 and a USB Product ID (PID) 0x1051.

🔎 If `cyme` doesn't work for any reason, you can use `cargo xtask usb-list`, which does the same thing but is much more basic. Run it from the root of the extracted tarball / git checkout:

```console
$ cargo xtask usb-list
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `xtask/target/debug/xtask usb-list`
Bus 002 Device 015: ID 1366:1051 <- J-Link on the nRF52840 Development Kit
(...) random other USB devices will be listed
```

✅ In the terminal run `cargo run --bin hello` from the [`nrf52-code/radio-app`](../../nrf52-code/radio-app) folder, to build and run a simple program on the DK to test the set-up.

```console
❯ cargo run --bin hello
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.09s
     Running `probe-rs run --chip=nRF52840_xxAA --allow-erase-all --log-format=oneline target/thumbv7em-none-eabihf/debug/hello`
      Erasing ✔ 100% [####################]  12.00 KiB @  18.51 KiB/s (took 1s)
  Programming ✔ 100% [####################]  12.00 KiB @  14.30 KiB/s (took 1s)
  Finished in 1.49s
Hello, world!
`dk::exit()` called; exiting ...
```

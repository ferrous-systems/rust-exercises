# nRF52 Tools

## VS Code

**Windows**: Go to [https://code.visualstudio.com](https://code.visualstudio.com) and run the installer.

**Linux**: Follow the instructions for your distribution on [https://code.visualstudio.com/docs/setup/linux](https://code.visualstudio.com/docs/setup/linux).

**macOS**: Go to [https://code.visualstudio.com](https://code.visualstudio.com) and click on "Download for Mac"

## OS specific dependencies

### Linux only: USB

Some of our tools depend on `pkg-config` and `libudev.pc`. Ensure you have the proper packages installed; on Debian based distributions you can use:

```console
sudo apt-get install libudev-dev libusb-1.0-0-dev
```

To access the USB devices as a non-root user, follow these steps:

1. (Optional) Connect the dongle and check its permissions with these commands:

    ```console
    $ lsusb -d 1915:521f
    Bus 001 Device 016: ID 1915:521f Nordic Semiconductor ASA USB Billboard
    $ #   ^         ^^

    $ # take note of the bus and device numbers that appear for you when run the next command
    $ ls -l /dev/bus/usb/001/016
    crw-rw-r-- 1 root root 189, 15 May 20 12:00 /dev/bus/usb/001/016
    ```

    The `root root` part in `crw-rw-r-- 1 root root` indicates the device can only be accessed by the `root` user.

2. Create the following file with the displayed contents. You'll need root permissions to create the file.

    ```console
    $ cat /etc/udev/rules.d/50-ferrous-training.rules
    # udev rules to allow access to USB devices as a non-root user

    # nRF52840 Dongle in bootloader mode
    ATTRS{idVendor}=="1915", ATTRS{idProduct}=="521f", TAG+="uaccess"

    # nRF52840 Dongle applications
    ATTRS{idVendor}=="1209", TAG+="uaccess"

    # nRF52840 Development Kit
    ATTRS{idVendor}=="1366", ENV{ID_MM_DEVICE_IGNORE}="1", TAG+="uaccess"
    ```

3. Run the following command to make the new udev rules effective

    ```console
    sudo udevadm control --reload-rules
    ```

4. (Optional) Disconnect and reconnect the dongle. Then check its permissions again.

    ```console
    $ lsusb
    Bus 001 Device 017: ID 1915:521f Nordic Semiconductor ASA 4-Port USB 2.0 Hub

    $ ls -l /dev/bus/usb/001/017
    crw-rw-r--+ 1 root root 189, 16 May 20 12:11 /dev/bus/usb/001/017
    ```

    The `+` part in `crw-rw-r--+` indicates the device can be accessed without `root` permissions.

### Windows only: Zadig JLink driver

On Windows you'll need to associate the nRF52840 Development Kit's USB device to the WinUSB driver.

To do that connect the nRF52840 DK to your PC using micro-USB port J2 (as done before) then download and run the [Zadig] tool.

[Zadig]: https://zadig.akeo.ie/

In Zadig's graphical user interface,

1. Select the 'List all devices' option from the Options menu at the top.

2. From the device (top) drop down menu select "BULK interface (Interface 2)"

3. Once that device is selected, `1366 1015` should be displayed in the USB ID field. That's the Vendor ID - Product ID pair.

4. Select 'WinUSB' as the target driver (right side)

5. Click "Install WinUSB driver". The process may take a few minutes to complete.

> You do not need to do anything for the **nRF52840 Dongle** device.

## Rust and tooling

### Base Rust installation

Go to [https://rustup.rs](https://rustup.rs/) and follow the instructions.

**Windows**: *Do* install the optional components of the [C++ build tools package](https://visualstudio.microsoft.com/visual-cpp-build-tools/). The installation size may take up to 2 GB of disk space.

### Rust Analyzer

**All**: Open VS Code and look for [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) in the marketplace (bottom icon in the left panel). Then install it.

**Windows**: It's OK to ignore the message about `git` not being installed, if you get one!

### Better TOML

**All**: For better handling of `Cargo.toml` files, we recommend you install [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml) if you're using VS Code.

### Rust Cross compilation support

**All**: Run this command in a terminal:

```console
rustup +stable target add thumbv7em-none-eabihf
```

### ELF analysis tools

**All**: Run these commands in a terminal:

```console
cargo install cargo-binutils
rustup +stable component add llvm-tools
```

### General purpose tools

Install the [`flip-link`](https://crates.io/crates/flip-link) and [`probe-run`](https://crates.io/crates/probe-run) tools using the following Cargo commands:

```console

$ cargo install probe-run
(..)
Installed package `probe-run v0.3.10` (..)

$ cargo install flip-link
(..)
Installed package `flip-link v0.1.7` (..)

$ cargo install nrfdfu
(..)
Installed package `nrfdfu v0.1.3` (..)
```

## Setup check

✅ Let's check that you have installed all the tools listed in the previous section.

```console
$ cargo size --version
cargo-size 0.3.6
```

✅ Connect the nRF52840-DK with your computer by plugging the usb-cable into the J2 connector on the DK (the usb connector on the short side of the board).

✅ In the terminal run the following command from the `nrf52-code/radio-app` folder. This will build and run a simple program on the DK to test the set-up.

```console
cargo run --bin hello
```

# nRF52 Radio Workbook

In this workshop you'll get familiar with:

- the structure of embedded Rust programs,
- the existing embedded Rust tooling, and
- embedded application development using a Hardware Abstraction Layer (HAL).

To put these concepts in practice you'll write applications that use the radio functionality of the nRF52840 microcontroller.

You have received two development boards for this workshop. We'll use both in the beginner workshop.

## The nRF52840 Development Kit

This is the larger development board.

The board has two USB ports: J2 and J3 and an on-board J-Link programmer / debugger -- [there are instructions to identify the ports in a previous section][id-ports]. USB port J2 is the J-Link's USB port. USB port J3 is the nRF52840's USB port. Connect the Development Kit to your computer using the **J2** port.

[id-ports]: ./nrf52-hardware.md#nrf52840-development-kit-dk

## The nRF52840 Dongle

This is the smaller development board.

The board has the form factor of a USB stick and can be directly connected to one of the USB ports of your PC / laptop. Do **not** connect it just yet.

## The nRF52840

Both development boards have an nRF52840 microcontroller. Here are some details about it that are relevant to this workshop.

- single core ARM Cortex-M4 processor clocked at 64 MHz
- 1 MB of Flash (at address `0x0000_0000`)
- 256 KB of RAM (at address `0x2000_0000`)
- IEEE 802.15.4 and BLE (Bluetooth Low Energy) compatible radio
- USB controller (device function)

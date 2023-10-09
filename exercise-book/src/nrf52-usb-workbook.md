# nRF52 USB Workbook

In this workshop you'll learn to:

- work with registers and peripherals from Rust
- handle external events in embedded Rust applications using RTIC
- debug event driven applications
- test `no_std` code

To put these concepts and techniques in practice you'll write a toy USB device application that gets enumerated and configured by the host. This embedded application will run in a fully event driven fashion: only doing work when the host asks for it.

You will need an nRF52840 Development Kit for this exercise, but not the nRF USB dongle.

## The nRF52840 Development Kit

The board has two USB ports: J2 and J3 and an on-board J-Link programmer / debugger -- [there are instructions to identify the ports in a previous section][id-ports]. USB port J2 is the J-Link's USB port. USB port J3 is the nRF52840's USB port. Connect the Development Kit to your computer using both ports.

[id-ports]: ./nrf52-hardware.md#nrf52840-development-kit-dk

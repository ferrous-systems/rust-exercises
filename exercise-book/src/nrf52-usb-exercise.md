# nRF52 USB Exercise

In this exercise you'll learn to:

- work with registers and peripherals from Rust
- handle external events in embedded Rust applications using RTIC
- debug event driven applications
- test `no_std` code

To put these concepts and techniques in practice you'll write a toy USB device application that gets enumerated and configured by the host. This embedded application will run in a fully event driven fashion: only doing work when the host asks for it.

You will need an nRF52840 Development Kit for this exercise, but not the nRF USB dongle.

## The nRF52840 Development Kit

The board has two USB ports: J2 and J3 and an on-board J-Link programmer / debugger -- [there are instructions to identify the ports in a previous section][id-ports]. USB port J2 is the J-Link's USB port. USB port J3 is the nRF52840's USB port. Connect the Development Kit to your computer using both ports.

[id-ports]: ./nrf52-hardware.md#nrf52840-development-kit-dk

## Exercise Steps

You will need to complete the exercise steps in order. It's OK if you don't get them all finished, but you must complete one before starting the next one. You can look at the solution for each step if you get stuck.

If you are reading the book view, the steps are listed on the left in the sidebar (use the hamburger if that is hidden). If you are reading the source on Github, go back to the [SUMMARY.md](./SUMMARY.md) file to see the steps.

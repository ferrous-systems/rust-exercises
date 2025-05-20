# nRF52 HAL Exercise

In this exercise you'll learn to:

- use a HAL to provide features in a BSP
- configure GPIO pins using the nRF52 HAL

To test your BSP changes, you will modify a small example: `hal-app/src/bin/blinky.rs`

You will need an nRF52840 Development Kit for this exercise, but not the nRF USB dongle.

If you haven't completed the Radio Exercise, you should start there, and go at least as far as completing the "Timers and Time" section.

## The nRF52840 Development Kit

This is the larger development board.

The board has two USB ports: J2 and J3 and an on-board J-Link programmer / debugger -- [there are instructions to identify the ports in a previous section][id-ports]. USB port J2 is the J-Link's USB port. USB port J3 is the nRF52840's USB port. Connect the Development Kit to your computer using the **J2** port.

[id-ports]: ./nrf52-hardware.md#nrf52840-development-kit-dk

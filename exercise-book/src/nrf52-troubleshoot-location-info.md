# `location info is incomplete` error

Problem: Using cargo run --bin hello from within the `nrf52-code/radio-app` folder finishes compiling and starts up probe-rs. But then the following error is returned:

```sh
Running `probe-rs run --chip nRF52840_xxAA target/thumbv7em-none-eabihf/debug/hello`
(HOST) WARN  (BUG) location info is incomplete; it will be omitted from the output
Error: AP ApAddress { dp: Default, ap: 0 } is not a memory AP
The LED5 next to the FTDI chip on the DK goes off for a split second but no program is flashed.
```

Solution: It seems like my nRF52840-DK was shipped with the MCU in some kind of protected state. Using nrfjprog from the nRF command line tools you can run nrfjprog --recover which makes the MCU exit this state and programming etc. using probe-rs works fine again.

Untested: using [nrf-recover](https://github.com/thalesfragoso/nrf-recover/) may also work.

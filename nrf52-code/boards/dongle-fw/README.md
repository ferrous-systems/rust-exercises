# nRF52840 USB Dongle Firmware

This folder is empty on Github, but in a release .zip archive it will contain:

* Puzzle Firmware - see <../../puzzle-fw>
* Loopback Firmware - see <../../loopback-fw>

## Loading Firmware

You can load one firmware image at a time into your USB dongle. Load the
firmware image with `nrfdfu`:

```console
$ cargo install nrfdfu
$ # Now press the dongle's 'Reset' button - the red LED should come on...
$ nrfdfu ./nrf52-code/boards/dongle-fw/puzzle-fw
[INFO  nrfdfu] Sending init packet...
[INFO  nrfdfu] Sending firmware image of size 37568...
[INFO  nrfdfu] Done.
```

## Compiling Firmware

Enter the source directory for the firmware, and run `cargo build --release`.
Note that if you compile your own puzzle firmware, you won't have the same
secret message as everyone else because it's not in the source code anywhere
(it's a Github Secret).

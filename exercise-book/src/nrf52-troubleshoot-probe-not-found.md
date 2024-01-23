# `no probe was found` error

You may encounter this error:

```console
Running probe-rs run --chip nRF52840_xxAA target/thumbv7em-none-eabihf/debug/hello
Error: no probe was found
```

- It may be caused by the micro-USB cable plugged on the long side of the board, instead of the short side.
- Check that the board is powered on.
- Check that your cable is a data cable and not power-only.

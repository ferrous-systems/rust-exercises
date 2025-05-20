# Dongle USB functionality is not working

> NOTE: this section only applies to the Radio exercise

If you don't get any output from `cargo xtask serial-term` it could just have been that first line got lost when re-enumerating the device from bootloader mode to the loopback application.

Run `cargo xtask serial-term` in one console window. Leave this window open.

In another window, run these *two* commands:

```console
$ cargo xtask change-channel 20
requested channel change to channel 20

$ cargo xtask change-channel 20
requested channel change to channel 20
```

If you get *two* lines of output in `cargo xtask serial-term` like this, you are good to go:

```console
$ cargo xtask serial-term
now listening on channel 20
now listening on channel 20
```

Return to the ["Interference"] section.

🔎 `cargo xtask serial-term` shows you the log output that the Dongle is sending to your computer via the serial interface (not over the wireless network!). After you've ran `cargo xtask change-channel`, it tells you that it is now listening for network traffic on channel 20. This is helpful for debugging, but not mission-critical.

["Interference"]: nrf52-radio-dongle.md#interference

If you only get one line of output then your OS may be losing some serial data -- we have seen this behavior on some macOS machines. You will still be able to work through the exercises but will miss log data every now and then. Return to the ["Interference"] section.

If you don't get *any* output from `cargo xtask serial-term` and/or the `cargo xtask change-channel` command fails then the Dongle's USB functionality is not working correctly. In this case you should ask your trainer for a custom firmware which has a fixed radio channel allocated. This means that when you program the Development Kit to send data to the Dongle, you need to ensure they are communicating on the same channel by setting

```rust ignore
/// make sure to pass the channel number of the loopback-nousb* program you
/// received from the trainer
radio.set_channel(Channel::_21);
```

Note that the `loopback-nousb*` programs do not send you any logs via `cargo xtask serial-term` for debugging but you will be able do the exercises nonetheless. For your debugging convenience, the Dongle will toggle the state of its green LED when it receives a packet. When you're done, return to the ["Interference"] section.

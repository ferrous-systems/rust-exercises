# `cargo run` errors

You may get one of these errors:

- "Access denied (insufficient permissions)" (seen on macOS)
- "USB error while taking control over USB device: Resource busy" (seen on Linux)

```console
$ cargo run --bin usb-4
Running `probe-run target/thumbv7em-none-eabihf/debug/usb-4`
Error: An error specific to a probe type occured: USB error while taking control over USB device: Access denied (insufficient permissions)

Caused by:
    USB error while taking control over USB device: Access denied (insufficient permissions)
```

```console
$ cargo run --bin usb-4
Running `probe-run target/thumbv7em-none-eabihf/debug/usb-4`
Error: An error specific to a probe type occured: USB error while taking control over USB device: Resource busy

Caused by:
    USB error while taking control over USB device: Resource busy
```

All of them have the same root issue: You have another instance of the `cargo run` process running.

It is not possible to have two or more instances of `cargo run` running. Terminate the old instance before executing `cargo run`. If you are using VS Code click the garbage icon ("Kill Terminal") on the top right corner of the terminal output window (located on the bottom of the screen).

# Radio In

In this section we'll explore the `recv_timeout` method of the Radio API. As the name implies, this is used to listen for packets. The method will block the program execution until a packet is received or the specified timeout has expired. We'll continue to use the Dongle in this section; it should be running the `loopback` application; and `cargo xtask serial-term` should also be running in the background.

The `loopback` application running on the Dongle will broadcast a radio packet after receiving one over channel 20. The contents of this outgoing packet will be the contents of the received one but reversed.

✅ Open the [`nrf52-code/radio-app/src/bin/radio-recv.rs`](../../nrf52-code/radio-app/src/bin/radio-recv.rs) file. Make sure that the Dongle and the Radio are set to the same channel. Click the "Run" button.

The Dongle does not inspect the contents of your packet and does not require them to be ASCII, or UTF-8. It will simply send a packet back containing the same bytes it received, except the bytes will be in reverse order to how you sent it.

That is, if you send `b"olleh"`, it will send back `b"hello"`.

The Dongle will respond as soon as it receives a packet. If you insert a delay between the `send` operation and the `recv` operation in the `radio-recv` program this will result in the DK not seeing the Dongle's response. So try this:

✅ Add a `timer.wait(x)` call before the `recv_timeout` call, where `x` is `core::time::Duration`; try different lengths of time for `x` and observe what happens.

Having log statements between `send` and `recv_timeout` can also cause packets to be missed so try to keep those two calls as close to each other as possible and with as little code in between as possible.

> NOTE Packet loss can always occur in wireless networks, even if the radios are close to each other. The `Radio` API we are using will not detect lost packets because it does not implement IEEE 802.15.4 Acknowledgement Requests. For the next step in the workshop, we will use a new function to handle this for us. For the sake of other radio users, please do ensure you never call `send()` in a tight loop!

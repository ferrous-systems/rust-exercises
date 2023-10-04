# Radio In

In this section we'll explore the `recv_timeout` method of the Radio API. As the name implies, this is used to listen for packets. The method will block the program execution until a packet is received or the specified timeout has expired. We'll continue to use the Dongle in this section; it should be running the `loopback` application; and `cargo xtask serial-term` should also be running in the background.

The `loopback` application running on the Dongle will broadcast a radio packet after receiving one over channel 20. The contents of this outgoing packet will be the contents of the received one but reversed.

✅ Open the `src/bin/radio-recv.rs` file. Make sure that the Dongle and the Radio are set to the same channel. Click the "Run" button.

The Dongle expects the packet to contain only ASCII characters and will not respond to packets that contain non-ASCII data. If you only send packets that contain byte string literals *with no escaped characters* (e.g. `b"hello"`) then this requirement will be satisfied. At the same time the Dongle will always respond with ASCII data so calling `str::from_utf8` on the response should never fail, unless the packet contents got corrupted in the transmission but the CRC should detect this scenario.

The Dongle will respond as soon as it receives a packet. If you insert a delay between the `send` operation and the `recv` operation in the `radio-recv` program this will result in the DK not seeing the Dongle's response. So try this:

✅ Add a `timer.delay(x)` call before the `recv_timeout` call; try different values of `x` and observe what happens.

Having log statements between `send` and `recv_timeout` can also cause packets to be missed so try to keep those two calls as close to each other as possible and with as little code in between as possible.

> NOTE Packet loss can always occur in wireless networks, even if the radios are close to each other. The `Radio` API we are using will not detect lost packets because it does not implement IEEE 802.15.4 Acknowledgement Requests. If you are having trouble with lost packets, consider adding a retry loop.

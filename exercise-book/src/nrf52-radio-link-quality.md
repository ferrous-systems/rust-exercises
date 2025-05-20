# Link Quality Indicator (LQI)

```console
received 7 bytes (CRC=Ok(0x2459), LQI=60)
```

✅ Now run the `radio-send` program several times with different variations to explore how LQI can be influenced

- change the distance between the Dongle and the DK -- move the DK closer to or further away from the Dongle.
- change the transmit power
- change the channel
- change the length of the packet
- different combinations of all of the above

Take note of how LQI changes with these changes. Does packet loss occur in any of these configurations?

> NOTE if you decide to send many packets in a single program then you should use the `Timer` API to insert a delay of at least five milliseconds between the transmissions. This is required because the Dongle will use the radio medium right after it receives a packet. Not including the delay will result in the Dongle missing packets

802.15.4 radios are often used in mesh networks like Wireless Sensors Networks (WSN). The devices, or *nodes*, in these networks can be mobile so the distance between nodes can change in time. To prevent a link between two nodes getting broken due to mobility the LQI metric is used to decide the transmission power -- if the metric degrades power should be increased, etc. At the same time, the nodes in these networks often need to be power efficient (e.g. are battery powered) so the transmission power is often set as low as possible -- again the LQI metric is used to pick an adequate transmission power.

## 🔎 802.15.4 compatibility

The radio API we are using follows the PHY layer of the IEEE 802.15.4 specification, but it's missing MAC level features like addressing (each device gets its own address), opt-in acknowledgment (a transmitted packet must be acknowledged with a response acknowledgment packet; the packet is re-transmitted if the packet is not acknowledged in time). These MAC level features are not implemented *in hardware* (in the nRF52840 Radio peripheral) so they would need to be implemented in software to be fully IEEE 802.15.4 compliant.

This is not an issue for these exercises but it's something to consider if you would like to continue from here and build a 802.15.4 compliant network API.

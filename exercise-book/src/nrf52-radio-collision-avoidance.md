# Collision avoidance

In this section you'll test the collision avoidance feature of the IEEE 802.15.4 radio used by the Dongle and DK.

If you check the API documentation of the `Radio` abstraction we have been using you'll notice that we haven't used these methods: `energy_detection_scan()`, `set_cca()` and `try_send()`.

The first method scans the currently selected channel (see `set_channel()`), measures the energy level of ongoing radio communication in this channel and returns the maximum energy observed over a span of time. This method can be used to determine what the *idle* energy level of a channel is. If there's non-IEEE 802.15.4 traffic on this channel the method will return a high value.

Under the 802.15.4 specification, before sending a data packet devices must first check if there's communication going on in the channel. This process is known as Clear Channel Assessment (CCA). The `send` method we have been used performs CCA in a loop and sends the packet only when the channel appears to be idle. The `try_send` method performs CCA *once* and returns the `Err` variant if the channel appears to be busy. In this failure scenario the device does not send any packet.

The `Radio` abstraction supports 2 CCA modes: `CarrierSense` and `EnergyDetection`. `CarrierSense` is the default CCA mode and what we have been using in this workshop. `CarrierSense` will only look for ongoing 802.15.4 traffic in the channel but ignore other traffic like 2.4 GHz WiFi and Bluetooth. The `EnergyDetection` method is able to detect ongoing non-802.15.4 traffic.

Here are some things for you to try out:

- First, read the section 6.20.12.4 of the [nRF52840 Product Specification](https://docs.nordicsemi.com/bundle/ps_nrf52840/page/keyfeatures_html5.html), which covers the nRF52840's implementation of CCA.

- Disconnect the Dongle. Write a program for the DK that scans and reports the energy levels of all valid 802.15.4 channels. In your location which channels have high energy levels when there's no ongoing 802.15.4 traffic? If you can, use an application like [WiFi Analyzer] to see which WiFi channels are in use in your location. Compare the output of WiFiAnalyzer to the values you got from  `energy_detection_scan`. Is there a correspondence? Note that WiFi channels don't match in frequency with 802.15.4 channels; some mapping is required to convert between them -- [check this illustration for more details about co-existence of 802.15.4 and WiFi][coexistence].

    [WiFi Analyzer]: https://play.google.com/store/apps/details?id=com.farproc.wifi.analyzer&hl=en
    [coexistence]: https://inet.omnetpp.org/docs/showcases/wireless/coexistence/doc/

- Choose the channel with the highest idle energy. Now write a program on the DK that sets the CCA mode to `EnergyDetection` and then send a packet over this channel using `try_send`. The `EnergyDetection` CCA mode requires a Energy Detection (ED) "threshold" value. Try different threshold values. What threshold value makes the `try_send` succeed?

- Repeat the previous experiment but use the channel with the lowest idle energy.

- Pick the channel with the lowest idle energy. Run the `loopback` app on the Dongle and set its listening channel to the chosen channel. Modify the DK program to perform a `send` operation immediately followed by a `try_send` operation. The `try_send` operation will collide with the response of the Dongle (remember: the Dongle responds to all incoming packets after a 5ms delay - see the `loopback-fw` program for details). Find a ED threshold that detects this collision and makes `try_send` return the `Err` variant.

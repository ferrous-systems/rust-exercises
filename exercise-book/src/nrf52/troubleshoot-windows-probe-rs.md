# Making `probe-rs` work on Windows

If `probe-rs` does not work on Windows, you might have to re-configure the firmware running on the on-board J-Link first
to use WinUSB. The [SEGGER website](https://kb.segger.com/J-Link_WinUSB_driver_selection) specifies
how you can do this using the [J-Link Software Package](https://www.segger.com/downloads/jlink/#J-LinkSoftwareAndDocumentationPack).

Alternatively, you can also install the [Zadig tool](https://zadig.akeo.ie/) and use it
to re-configure WinUSB as the driver for the probe.

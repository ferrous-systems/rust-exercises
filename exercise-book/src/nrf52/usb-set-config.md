# SET_CONFIGURATION (Linux & macOS)

On Linux and macOS, the host will likely send a SET_CONFIGURATION request right after enumeration to put the device in the `Configured` state. For now you can stall the request. It is not necessary at this stage because the device has already been enumerated.

# Checking the API documentation

We'll be using the `dk` Board Support Package. It's good to have its API documentation handy. You can generate the documentation for that crate from the command line:

✅ Run the following command from within the `nrf52-code/usb-app` folder. It will open the generated documentation in your default web browser. Note that if you run it from inside the `nrf52-code/boards/dk` folder, you will find a bunch of USB-related documentation missing, because we disable that particular feature by default.

```console
cargo doc --open
```

> NOTE: If you are using Safari and the documentation is hard to read due to missing CSS, try opening it in a different browser.

✅ Browse to the documentation for the `dk` crate, and look at what is available within the `usbd` module. Some of these functions will be useful later.

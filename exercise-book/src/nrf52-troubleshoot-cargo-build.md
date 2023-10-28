# `cargo build` fails to link

If you have configured Cargo to use sccache then you'll need to disable sccache support. Unset the `RUSTC_WRAPPER` variable in your environment *before* opening VS code. Run `cargo clean` from the Cargo workspace you are working from (`nrf52-code/radio-app` or `nrf52-code/usb-app`). Then open VS code.

If you are on Windows and get linking errors like `LNK1201: error writing to program database`, then something in your target folder has become corrupt. A `cargo clean` should fix it.

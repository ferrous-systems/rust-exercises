# References and Resources

## Radio Project

- [nRF52840 Product Specification](https://docs.nordicsemi.com/bundle/ps_nrf52840/page/keyfeatures_html5.html)
- The [Embedded Rust Book][embedded rust] is a great learning resource, especially the Concurrency chapter.
- If you are looking to write an interrupt handler, look at the [`#[interrupt]` attribute][interrupt]. All interrupts implemented by the nrf52840 HAL are listed in [`nrf52840-pac/src/lib.rs`][pac]. It is also recommended that you work through the USB exercise to learn about [RTIC][rtic].

[pac]: https://github.com/nrf-rs/nrf52840-pac/blob/9558a3ed032b2aec7e57c2f42330f1dee0000a04/src/lib.rs#L167
[interrupt]: https://docs.rs/cortex-m-rt/0.7.5/cortex_m_rt/attr.interrupt.html
[rtic]: https://rtic.rs/2/book/en/
[embedded rust]: https://rust-embedded.github.io/book/

## USB Project

- [nRF52840 Product Specification](https://docs.nordicsemi.com/bundle/ps_nrf52840/page/keyfeatures_html5.html)
- [Universal Serial Bus (USB) Specification Revision 2.0](https://www.usb.org/document-library/usb-20-specification)

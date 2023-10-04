# Interrupt handling

We haven't covered interrupt handling in the workshop but the `cortex-m-rt` crate provides attributes to declare exception and interrupt handlers: `#[exception]` and `#[interrupt]`. You can find documentation about these attributes and how to safely share data with interrupt handlers using Mutexes in the ["Concurrency" chapter][concurrency] of the Embedded Rust book.

Another way to deal with interrupts is to use a framework like Real-Time Interrupt-driven Concurrency ([RTIC]); this framework has a [book] that explains how you can build reactive applications using interrupts. We use this framework in the advanced level workshop.

[concurrency]: https://rust-embedded.github.io/book/concurrency/index.html
[RTIC]: https://crates.io/crates/cortex-m-rtic
[book]: https://rtic.rs/0.5/book/en/

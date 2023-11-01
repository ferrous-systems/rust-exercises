# Stack Overflow Protection

The `usb-app` crate in which we developed our advanced workshop solutions (i.e. `nrf52-code/usb-app`) uses our open-source [`flip-link`] tool for zero-cost stack overflow protection.

This means that your application will warn you by crashing if you accidentally overreach the boundaries of your application's stack instead of running into *undefined behavior* and behaving erratically in irreproducible ways. This memory protection mechanism comes at no additional computational or memory-usage cost.

🔎  For a detailed description of how `flip-link` and Stack Overflows in bare metal Rust in general work, please refer to the [`flip-link` README].

You can see this in action in the `stack_overflow.rs` file that can be found in `nrf52-code/usb-app/src/bin/stack_overflow.rs`:

```rust ignore
{{#include ../../nrf52-code/usb-app/src/bin/stack_overflow.rs}}
```

The `spam()` function allocates data on the stack until the stack boundaries are reached.

✅ Run `stack_overflow.rs`

You should see output similar to this (the program output between the horizontal bars might be missing):

```console
(HOST) INFO  flashing program (35.25 KiB)
(HOST) INFO  success!
────────────────────────────────────────────────────────────────────────────────
INFO:stack_overflow -- provoking stack overflow...
INFO:stack_overflow -- address of current `use_stack` at recursion depth 0: 0x2003aec0
INFO:stack_overflow -- address of current `use_stack` at recursion depth 1: 0x20039e50
(...)
INFO:stack_overflow -- address of current `use_stack` at recursion depth 10: 0x20030a60
INFO:stack_overflow -- address of current `use_stack` at recursion
────────────────────────────────────────────────────────────────────────────────
stack backtrace:
   0: HardFaultTrampoline
      <exception entry>
(HOST) WARN  call stack was corrupted; unwinding could not be completed
(HOST) ERROR the program has overflowed its stack
```

❗️ `flip-link` is a third-party tool, so make sure you've installed it through `cargo install flip-link`

To see how we've activated `flip-link`, take a look at `nrf52-code/usb-app/.cargo/config.toml`:

```toml
rustflags = [
  "-C", "linker=flip-link", # adds stack overflow protection
  #
]
```

There, we've configured `flip-link` as the linker to be used for all ARM targets. If you'd like to use `flip-link` in your own projects, this is all you need to add!

🔎  Note: if you try to run `stack_overflow.rs` *without* `flip-link` enabled, you might see varying behavior depending on the `rustc` version you're using, timing and pure chance. This is because undefined behavior triggered by the program may change between `rustc` releases.

[`flip-link`]: https://github.com/knurling-rs/flip-link
[`flip-link` README]: https://github.com/knurling-rs/flip-link/blob/main/README.md

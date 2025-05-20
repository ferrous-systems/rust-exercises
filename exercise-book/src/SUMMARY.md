<!-- markdownlint-disable MD025 -->
# Summary

[Introduction](./introduction.md)

# Rust Fundamentals

- [Fizzbuzz](./fizzbuzz.md)
  - [Fizzbuzz Cheat-Sheet](./fizzbuzz-cheat-sheet.md)
- [Fizzbuzz with match](./fizzbuzz-match.md)
- [Rust Latin](./rustlatin.md)
- [URLs, match, result](./urls-match-result.md)
- [Calculator](./calculator.md)
- [Iterators](./iterators.md)
- [SimpleDB](./simple-db.md)
  - [Knowledge](./simple-db-knowledge.md)
  - [Step-by-Step Solution](./simple-db-solution.md)
- [Green and Yellow game](./green-yellow-game.md)

# Applied Rust

- [Shapes](shapes.md)
- [Connected Mailbox](./connected-mailbox.md)
- [Multithreaded mailbox](./multi-threaded-mailbox.md)

# Self-check Project

- [Self-check project](./self-check.md)

# Working with the nRF52

- [Preparation](./nrf52-preparation.md)
  - [Code Organization](./nrf52-code-organisation.md)
  - [Hardware](./nrf52-hardware.md)
  - [Software Tools](./nrf52-tools.md)
- [References and Resources](./nrf52-references-resources.md)
  - [Tooltips](./nrf52-tooltips.md)
- [Troubleshooting](./nrf52-troubleshooting.md)
  - [`cargo-size` is not working](./nrf52-troubleshoot-cargo-size.md)
  - [Rust analyzer is not working](./nrf52-troubleshoot-rust-analyzer.md)
  - [`cargo build` fails to link](./nrf52-troubleshoot-cargo-build.md)
  - [Dongle USB functionality is not working](./nrf52-troubleshoot-usb-dongle.md)
  - [`cargo run` errors](./nrf52-troubleshoot-cargo-run-error.md)
  - [`no probe was found` error](./nrf52-troubleshoot-probe-not-found.md)
  - [`location info is incomplete` error](./nrf52-troubleshoot-location-info.md)

# Bare-Metal Rust: Getting Started

- [nRF52 Radio Exercise](./nrf52-radio-exercise.md)
  - [Parts of an Embedded Program](./nrf52-radio-parts-embedded-program.md)
  - [Building an Embedded Program](./nrf52-radio-building-program.md)
  - [Binary Size](./nrf52-radio-binary-size.md)
  - [Running the Program](./nrf52-radio-running-from-vsc.md)
  - [Panicking](./nrf52-radio-panicking.md)
  - [Using a Hardware Abstraction Layer](./nrf52-radio-using-hal.md)
  - [Timers and Time](./nrf52-radio-time.md)
  - [nRF52840 Dongle](./nrf52-radio-dongle.md)
  - [Radio Out](./nrf52-radio-out.md)
    - [Radio Setup](./nrf52-radio-setup.md)
    - [Messages](./nrf52-radio-messages.md)
    - [Link Quality Indicator (LQI)](./nrf52-radio-link-quality.md)
  - [Radio In](./nrf52-radio-in.md)
  - [Radio Puzzle](./nrf52-radio-puzzle.md)
  - [Radio Puzzle Help](./nrf52-radio-puzzle-help.md)
  - [Next Steps](./nrf52-radio-next-steps.md)
    - [Alternative containers](./nrf52-radio-alt-containers.md)
    - [Collision avoidance](./nrf52-radio-collision-avoidance.md)
    - [Interrupt handling](./nrf52-radio-interrupt-handling.md)
    - [Starting a Project from Scratch](./nrf52-radio-from-scratch.md)

# Bare-Metal Rust: Using a HAL

- [nRF52 HAL Exercise](./nrf52-hal-exercise.md)
  - [Adding Buttons](./nrf52-hal-buttons.md)

# Bare-Metal Rust: Interrupts

- [nRF52 USB Exercise](./nrf52-usb-exercise.md)
  - [Listing USB Devices](./nrf52-usb-listing-usb-devices.md)
  - [Hello, world!](./nrf52-usb-hello-world.md)
  - [Checking the API documentation](./nrf52-usb-api-documentation.md)
  - [RTIC hello](./nrf52-usb-rtic-hello.md)
  - [Dealing with Registers](./nrf52-usb-dealing-with-registers.md)
  - [Event Handling](./nrf52-usb-event-handling.md)
  - [Task State](./nrf52-usb-task-state.md)
  - [USB Enumeration](./nrf52-usb-usb-enumeration.md)
  - [USB-1: Dealing with USB Events](./nrf52-usb-usb-events.md)
  - [USB Endpoints](./nrf52-usb-usb-endpoints.md)
  - [USB Control Transfers](./nrf52-usb-control-transfers.md)
  - [USB-2: SETUP Stage](./nrf52-usb-setup-stage.md)
  - [USB Device Descriptors](./nrf52-usb-device-descriptor.md)
  - [USB-3: DATA Stage](./nrf52-usb-data-stage.md)
  - [USB Configuration Descriptors](./nrf52-usb-configuration-descriptor.md)
  - [USB-4: Supporting more Standard Requests](./nrf52-usb-supporting-standard-requests.md)
  - [USB-5: Getting it Configured](./nrf52-usb-getting-device-configured.md)
  - [USB-5: Idle State](./nrf52-usb-idle-state.md)
  - [Next Steps](./nrf52-usb-advanced-next-steps.md)
  - [Extra Info](./nrf52-usb-extra-info.md)
    - [The USB Specification](./nrf52-usb-usb-specification.md)
    - [DMA](./nrf52-usb-dma.md)
    - [SET_CONFIGURATION (Linux & macOS)](./nrf52-usb-set-config.md)
    - [USB Interfaces](./nrf52-usb-interfaces.md)
    - [USB Interface descriptor](./nrf52-usb-interface-descriptor.md)
    - [USB Endpoint descriptor](./nrf52-usb-endpoint-descriptor.md)
    - [Inspecting the Descriptors](./nrf52-usb-inspecting-descriptors.md)
    - [Stack Overflow Protection](./nrf52-usb-stack-overflow-protection.md)

# Rust for Real-Time Systems

## Working without std

- [Working without std](./realtime-withoutstd.md)
  - [Replacing println!](./realtime-withoutstd-println.md)

## Bare-Metal Firmware on Cortex-R52

- [Bare-Metal Firmware on Cortex-R52](./realtime-v8r-preparation.md)
  - [UART Driver](./realtime-v8r-uart.md)

## Rust for Linux

- [Building a Linux Kernel Driver using Rust](./building-linux-kernel-driver.md)

# Async Rust

- [Interactive TCP Echo Server](./tcp-server.md)
  - [Share data between connections](./tcp-server-log.md)

# Async chat

- [Implementing a chat](./async-chat/index.md)
  - [Specification and Getting started](./async-chat/specification.md)
  - [Writing an Accept Loop](./async-chat/accept_loop.md)
  - [Receiving Messages](./async-chat/receiving_messages.md)
  - [Sending Messages](./async-chat/sending_messages.md)
  - [A broker as a connection point](./async-chat/connecting_readers_and_writers.md)
  - [Glueing all together](./async-chat/all_together.md)
  - [Clean Shutdown](./async-chat/clean_shutdown.md)
  - [Handling Disconnection](./async-chat/handling_disconnection.md)
  - [Final Server Code](./async-chat/final_server_code.md)
  - [Implementing a Client](./async-chat/implementing_a_client.md)

# Kani Rust Verifier

- [Verifying Data Structures with Kani](./kani-linked-list.md)

# Other Topics

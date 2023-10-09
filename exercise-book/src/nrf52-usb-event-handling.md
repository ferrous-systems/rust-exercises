# Event Handling

Below the `idle` function you'll see a `#[task]` handler, a function. This *task* is bound to the POWER_CLOCK interrupt signal and will be executed, function-call style, every time the interrupt signal is raised by the hardware.

✅ Run the `events` application. Then connect a micro-USB cable to your PC/laptop then connect the other end to the DK (port J3). You'll see the "POWER event occurred" message after the cable is connected.

Note that all tasks will be prioritized over the `idle` function so the execution of `idle` will be interrupted (paused) by the `on_power_event` task. When the `on_power_event` task finishes (returns) the execution of the `idle` will be resumed. This will become more obvious in the next section.

Try this: add an infinite loop to the end of `init` so that it never returns. Now run the program and connect the USB cable. What behavior do you observe? How would you explain this behavior? (hint: look at the `rtic-expansion.rs` file: under what conditions is the `init` function executed?)

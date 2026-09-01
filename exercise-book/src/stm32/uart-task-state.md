# Task State

Let's fix our interrupt handler so that it doesn't run continuously. To do that, we need to read the character from the UART, causing the FIFO to empty and the interrupt to stop being asserted.

To do *that* we need to have access to our UART in our interrupt handler.

✅ Open the same [`stm32-code/standalone-app/src/bin/standalone-rtic-event.rs`](../../../stm32-code/standalone-app/src/bin/standalone-rtic-event.rs) file as before.

Tasks run from start to finish, like functions, in response to events. To preserve some state between the different executions of a task we can add a *resource* to the task. In RTIC, resources are the mechanism used to *share* data between different tasks in a memory safe manner but they can also be used to hold task state.

To get the desired behavior we'll want to store the UART in the state of the `usart1_handler` task.

The starter code shows the syntax to declare a resource, the `Resources` struct, and the syntax to associate a resource to a task, the `resources` list in the `#[task]` attribute.

In the starter code a resource is used to *move* (by value) the USART1 driver from `init` to the `usart1_handler` task. The USART1 driver then becomes part of the state of the `usart1_handler` task and can be persistently accessed throughout calls to `usart1_handler()` through a *mutable reference*. The resources of a task are available via the `Context` argument of the task.

To elaborate more on this *move* action: we only want to have a single USART1 driver because it represents ownership of some hardware. The consequence of this design is that having ownership of an object like `usart::Driver` means that the function (or task) has exclusive access, or ownership, over the peripheral. This is the case of the `init` function: it is given the USART driver as part of the `NonSecureBoard` object and then transfers ownership of it over to a task, using the resource initialization mechanism.

We have moved the USART1 driver into the task because we want to clear the interrupt by reading a character from the UART. If we miss this step the `usart1_handler` task (function) will be called again once it returns and then again and again and again (ad infinitum).

✅ Modify the program so that it reads characters from the UART, and logs what it read

```console
[INFO] USART1 IRQ!
[INFO] < 0x31
[INFO] < 0x30
[INFO] USART1 IRQ!
[INFO] < 0x48
```


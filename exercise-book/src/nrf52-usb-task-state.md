# Task State

Now let's say we want to change the previous program to count how many times the USB cable (port J3) has been connected and disconnected.

✅ Open the `nrf52-code/usb-app/src/bin/task-state.rs` file.

Tasks run from start to finish, like functions, in response to events. To preserve some state between the different executions of a task we can add a *resource* to the task. In RTIC, resources are the mechanism used to *share* data between different tasks in a memory safe manner but they can also be used to hold task state.

To get the desired behavior we'll want to store some counter in the state of the `on_power_event` task.

The starter code shows the syntax to declare a resource, the `Resources` struct, and the syntax to associate a resource to a task, the `resources` list in the `#[task]` attribute.

In the starter code a resource is used to *move* (by value) the POWER peripheral from `init` to the `on_power_event` task. The POWER peripheral then becomes part of the state of the `on_power_event` task and can be persistently accessed throughout calls to `on_power_event()` through a *reference*. The resources of a task are available via the `Context` argument of the task.

To elaborate more on this *move* action: in the `svd2rust` API, peripheral types like `POWER` are *singletons* (only a single value of that type can ever exist). The consequence of this design is that holding a peripheral instance, like `POWER`, *by value* means that the function (or task) has exclusive access, or ownership, over the peripheral. This is the case of the `init` function: it owns the `POWER` peripheral but then transfers ownership over it to a task using the resource initialization mechanism.

We have moved the POWER peripheral into the task because we want to clear the USBDETECTED interrupt flag after it has been set by the hardware. If we miss this step the `on_power_event` task (function) will be called again once it returns and then again and again and again (ad infinitum).

Also note that in the starter code the `idle` function has been modified. Pay attention to the logs when you run the starter code.

✅ Modify the program so that it prints the number of times the USB cable has been connected to the DK every time the cable is connected, as shown below.

```console
USBDETECTED interrupt enabled
idle: going to sleep
on_power_event: cable connected 1 time
idle: woke up
idle: going to sleep
on_power_event: cable connected 2 times
idle: woke up
idle: going to sleep
on_power_event: cable connected 3 times
```

You can find a solution to this exercise in the `nrf52-code/usb-app-solutions/src/bin/task-state.rs` file.

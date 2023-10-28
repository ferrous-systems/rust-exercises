# Error handling

Since the logic of the `EP0SETUP` event handling is getting more complex with each added event, you can see that `usb-4.rs` was refactored to add error handling: the event handling now happens in a separate function *that returns a `Result`*. When it encounters an invalid host request, it returns the `Err` variant which can be handled by stalling the endpoint:

```rust ignore
fn on_event(/* parameters */) {
    match event {
        /* ... */
        Event::UsbEp0Setup => {
            if ep0setup(/* arguments */).is_err() {
                // unsupported or invalid request:
                // TODO add code to stall the endpoint
                defmt::warn!("EP0IN: unexpected request; stalling the endpoint");
            }
        }
    }
}

fn ep0setup(/* parameters */) -> Result<(), ()> {
    let req = Request::parse(/* arguments_*/)?;
    //                                       ^ early returns an `Err` if it occurs

    // TODO respond to the `req`; return `Err` if the request was invalid in this state

    Ok(())
}
```

Note that there's a difference between the error handling done here and the error handling commonly done in `std` programs. `std` programs usually bubble up errors to the top `main` function (using the `?` operator), report the error (or chain of errors) and then exit the application with a non-zero exit code. This approach is usually not appropriate for embedded programs as  

1. `main` cannot return,  
2. there may not be a console to print the error to and/or  
3. stopping the program, and e.g. requiring the user to reset it to make it work again, may not be desirable behavior.  

For these reasons in embedded software errors tend to be handled as early as possible rather than propagated all the way up.

This does not preclude error *reporting*. The above snippet includes error reporting in the form of a `defmt::warn!` statement. This log statement may not be included in the final release of the program as it may not be useful, or even visible, to an end user but it is useful during development.

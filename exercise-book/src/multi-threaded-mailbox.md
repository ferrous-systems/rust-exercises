# Multi-Threaded Mailbox Exercise

In this exercise, we will take our "Conneted Mailbox" and make it multi-threaded. A new thread should be spawned for every incoming connection, and that thread should take ownership of the `TcpStream` and drive it to completion.

## After completing this exercise you are able to

- spawn threads

- convert a non-thread-safe type into a thread-safe-type

- lock a Mutex to access the data within

## Prerequisites

- A completed "Connected Mailbox" solution

## Tasks

1. Use the `std::thread::spawn` API to start a new thead when your main loop produces a new connection to a client. The `handle_client` function should be executed within that spawned thread. Note how Rust doesn't let you pass `&mut VecDequeue<String>` into the spawned thread, both becuase you have multiple `&mut` references (not allowed) and because the thread might live longer than the `VecDeque` (which only lives whilst the `main()` function is running, and `main()` might quit at any time with an early return or a break out of the connection loop).

2. Convert the `VecDeque` into a `Arc<Mutex<VecDequeue>>` (use `std::sync::Mutex`). Change the `handle_client` function to take a `&Mutex<VecDeque>`. Clone the Arc handle with `.clone()` and `move` that cloned handle into the new thread. Change the `handle_client` function to call `let mut queue = your_mutex.lock().unwrap();` whenever you want to access the queue inside the Mutex.

3. Convert the `Arc<Mutex<VecDeque>>` into a `Mutex<VecDeque>` and introduce scoped threads with `std::thread::scope`. The `Mutex<VecDeque>` should be created outside of the scope (ensure it lives longer than any of the scoped threads), but the connection loop should be inside the scope. Change `std::thread::spawn` to be `s.spawn`, where `s` is the name of the argument to the scope closure.

At every step (noting that Step 1 won't actually work...), try out your program using a command-line TCP Client: you can either use `nc`, or `netcat`, or our supplied `tools/tcp-client` program.

## Optional Tasks:

- Run `cargo clippy` on your codebase.
- Run `cargo fmt` on your codebase.

## Help

### Making a Arc, containing a Mutex, containing a VecDeque

You can just nest the calls to `SomeType::new()`...

<details>
  <summary>Solution</summary>

```rust
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

fn main() {
    // This type annotation isn't required if you actually push something into the queue...
    let queue_handle: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));
}
```

</details>

### Spawning Threads

The `std::thread::spawn` function takes a closure. Rust will automatically try and borrow any local variables that the closure refers to but that were declared outside the closure. You can put `move` in front of the closure bars (e.g. `move ||`) to make Rust try and take ownership of variables instead of borrowing them.

You will want to clone the `Arc` and move the clone into the thread.

<details>
  <summary>Solution</summary>

```rust
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

fn main() {
    let queue_handle = Arc::new(Mutex::new(VecDeque::new()));

    for _ in 0..10 {
        // Clone the handle and move it into a new thread
        let thread_queue_handle = queue_handle.clone();
        std::thread::spawn(move || {
            handle_client(&thread_queue_handle);
        });

        // This is the same, but fancier. It stops you passing the wrong Arc handle
        // into the thread.
        std::thread::spawn({ // this is a block expression
            // This is declared inside the block, so it shadows the one from the
            // outer scope.
            let queue_handle = queue_handle.clone();
            // this is the closure produced by the block expression
            move || {
                handle_client(&queue_handle);
            }
        });
    }

    // This doesn't need to know it's in an Arc, just that it's in a Mutex.
    fn handle_client(locked_queue: &Mutex<VecDeque<String>>) {
        todo!();
    }
}
```

</details>

## Locking a Mutex

A value of type `Mutex<T>` has a `lock()` method, but this method can fail if the Mutex has been poisoned (i.e. a thread panicked whilst holding the lock). We generally don't worry about handling the poisoned case (because one of your threads has already panicked, so the program is in a fairly bad state already), so we just use `unwrap()` to make this thread panic as well.

<details>
  <summary>Solution</summary>

```rust
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

fn main() {
    let queue_handle = Arc::new(Mutex::new(VecDeque::new()));

    let mut inner_q = queue_handle.lock().unwrap();
    inner_q.push_back("Hello".to_string());
    println!("{:?}", inner_q.pop_front());
    println!("{:?}", inner_q.pop_front());
}
```
</details>

## Creating a thread scope.

The 

<details>
    <summary>Solution</summary>

```rust
use std::collections::VecDeque;
use std::sync::Mutex;

fn main() {
    let locked_queue = Mutex::new(VecDeque::new());

    std::thread::scope(|s| {
        for i in 0..10 {
            let locked_queue = &locked_queue;
            s.spawn(move || {
                let mut inner_q = locked_queue.lock().unwrap();
                inner_q.push_back(i.to_string());
                println!("Pop {:?}", inner_q.pop_front());
            });
        }
    });
}
```

</details>

### Solution

If you need it, we have provided a [complete solution](../../exercise-solutions/multi-threaded-mailbox) for this exercise.

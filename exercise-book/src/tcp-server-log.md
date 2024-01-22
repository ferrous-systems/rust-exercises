# Share data between connections

In this exercise we will take our interactive server and will add a common log for *lengths of messages* that each client sends us. We will explore synchronization primitives that Rust offers in its Standard Library.

## After completing this exercise you are able to

- share data between threads using `Mutex`es

- use reference-counting to ensure data stays available across multiple threads

- use scoped threads to avoid runtime reference counting

- use channels and message passing to share data among threads by communicating

# Tasks

## Part 1

1. Add a log to store length of messages: `let mut log: Vec<usize> = vec![];`
2. Pass it to a `handle_client` function and record a length of each incoming line of text:
    ```rust
    log.push(line.len());
    ```
3. Resolve lifetime issues by using a reference-counting pointer.
4. Resolve mutability issues by using a mutex

## Part 2

5. Use [`thread::scope`](https://doc.rust-lang.org/stable/std/thread/fn.scope.html) function to get rid of reference counting for `log` vector

## Part 3

6. Instead of sharing `log` vector use a [`mpsc::channel`](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html) to send length of lines from worker threads.
7. Create a separate thread that listens for new channel messages and updates the vector accordingly.

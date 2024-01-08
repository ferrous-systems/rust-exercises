<!-- markdownlint-disable MD033 MD014 -->
# Self-Check Project

*This exercise is intended for you to check your Rust knowledge. It is based on our other exercises, so you can follow them one by one instead of attempting to do everything in one go.*

In this exercise we will create a small in-memory message queue that is accessible over a TCP connection and uses a plain text format for its protocol. The protocol has two commands to put a message into the back of the queue and to read a message from the front of the queue. When a user sends a "PUBLISH" we will push the data into a queue, and when the user sends a "RETRIEVE" we will pop some data off the queue (if any is available). The user will connect via TCP to port 7878. Multiple clients can add and remove messages from the queue at the same time.

## After completing this exercise you are able to

- write a Rust binary that uses a Rust library

- combine two Rust packages into a Cargo workspace

- open a TCP port and perform an action when each user connects

- use I/O traits to read/write from a TCP socket

- create a safe protocol parser in Rust manually

- interact with borrowed and owned memory, especially how to take ownership

- handle complex cases using the `match` and `if let` syntax

- handle errors using `Result` and custom error types

- spawn threads

- convert a non-thread-safe type into a thread-safe-type

- lock a Mutex to access the data within

## Tasks

1. Create a Cargo workspace for your project.
2. Create a binary package inside your workspace for your TCP server
3. Implement a simple TCP Server that listens for connections on `127.0.0.1:7878`. For each incoming connection, read all of the input as a string, and send it back to the client.
4. Create a library package inside your workspace for a message protocol parser. Make your TCP server depend on that library using a relative path.
5. Inside your library implement the following function so that it implements the protocol specifications to parse the messages. Use the provided tests to help you with the case handling.

    ```rust, ignore
    pub fn parse(input: &str) -> Result<Command, Error> {
        todo!()
    }
    ```

6. Change your TCP Server to *use* your parser crate to parse the input, and provide an appropriate canned response.
7. Set up a `VecDeque` queue and either *push* or *pop* from that queue, depending on the command you have received.
8. Add support for multiple simultaneous client connections using threads. Make sure all clients read a write to the same shared queue.

### Optional Tasks

- Allow each connection to read input line by line as a sequence of commands and execute them in the same order as they come in. This way you should be able to open several connections in terminal and type commands in them one by one.
- Run `cargo fmt` on your codebase.
- Run `cargo clippy` on your codebase.

### Protocol Specification

The protocol has two commands that are sent as messages in the following
form:

- `PUBLISH <payload>\n`

- `RETRIEVE\n`

With the additional properties:

1. The payload cannot contain newlines.

2. A missing newline at the end of the command is an error.

3. Data after the first newline is an error.

4. Empty payloads are allowed. In this case, the command is
    `PUBLISH \n`.

Violations against the form of the messages and the properties are
handled with the following error codes:

- `TrailingData` (bytes found after newline)

- `IncompleteMessage` (no newline)

- `EmptyMessage` (empty string instead of a command)

- `UnknownCommand` (string is not empty, but neither `PUBLISH` nor
    `RECEIVE`)

- `UnexpectedPayload` (message contains a payload, when it should not)

- `MissingPayload` (message is missing a payload)

### Testing

Below are the tests your protocol parser needs to pass. You can copy them to the bottom of your `lib.rs`.

```rust, ignore
#[cfg(test)]
mod tests {
    use super::*;

    // Tests placement of \n
    #[test]
    fn test_missing_nl() {
        let line = "RETRIEVE";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::IncompleteMessage);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_trailing_data() {
        let line = "PUBLISH The message\n is wrong \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::TrailingData);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_string() {
        let line = "";
        let result = parse(line);
        let expected = Err(Error::IncompleteMessage);
        assert_eq!(result, expected);
    }

    // Tests for empty messages and unknown commands

    #[test]
    fn test_only_nl() {
        let line = "\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::EmptyMessage);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_unknown_command() {
        let line = "SERVE \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnknownCommand);
        assert_eq!(result, expected);
    }

    // Tests correct formatting of RETRIEVE command

    #[test]
    fn test_retrieve_w_whitespace() {
        let line = "RETRIEVE \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnexpectedPayload);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_retrieve_payload() {
        let line = "RETRIEVE this has a payload\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnexpectedPayload);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_retrieve() {
        let line = "RETRIEVE\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Ok(Command::Retrieve);
        assert_eq!(result, expected);
    }

    // Tests correct formatting of PUBLISH command

    #[test]
    fn test_publish() {
        let line = "PUBLISH TestMessage\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Ok(Command::Publish("TestMessage".into()));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_publish() {
        let line = "PUBLISH \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Ok(Command::Publish("".into()));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_missing_payload() {
        let line = "PUBLISH\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::MissingPayload);
        assert_eq!(result, expected);
    }
}
```

## Help

### Making a Workspace

<details>
  <summary>Solution</summary>
A workspace file looks like:

```toml
[workspace]
resolver= "2"
members = ["command-parser", "tcp-server"]
```

Each member is a folder containing a Cargo package (i.e. that contains a `Cargo.toml` file).
</details>

### Connecting over TCP/IP

#### Using `nc`, `netcat` or `ncat`

The `nc`, `netcat`, or `ncat` tools may be available on your macOS or Linux machine, or on WSL on windows. They all work in a similar fashion.

```console
$ echo "PUBLISH 1234" | nc 127.0.0.1 7878
```

The `echo` command adds a new-line character automatically. Use `echo -n` if you don't want it to add a new-line character.

#### Using our TCP Client

We have written a basic TCP Client which should work on any platform.

```console
$ cd tools/tcp-client
$ cargo run -- "PUBLISH hello"
$ cargo run -- "RETRIEVE"
```

It automatically adds a *newline* character on to the end of every message you send. It is hard-coded to connect to a server at `127.0.0.1:7878`.

### Writing to a stream

If you want to write to an object that implements `std::io::Write`, you could use `writeln!`.

<details>
  <summary>Solution</summary>

```rust
use std::io::prelude::*;
use std::net::{TcpStream};

fn handle_client(mut stream: TcpStream) -> Result<(), std::io::Error> {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;
    println!("Received: {:?}", buffer);
    writeln!(stream, "Thank you for {buffer:?}!")?;
    Ok(())
}
```

</details>

### Writing a TCP Server

If you need a working example of a basic TCP Echo server, you can start with [our template](../../exercise-templates/tcp-echo-server).

<details>
  <summary>Solution</summary>

```rust no_run
{{#include ../../exercise-templates/tcp-echo-server/src/main.rs}}
```

</details>

### Handling Errors

<details>
  <summary>Solution</summary>

In a binary program `anyhow` is a good way to handle top-level errors.

```rust ignore
use std::io::Read;

fn handle_client(stream: &mut std::net::TcpStream) -> Result<(), anyhow::Error> {
    // This returns a `Result<(), std::io::Error>`, and the `std::io::Error` will auto-convert into an `anyhow::Error`.
    stream.read_to_string(&mut buffer)?;
    /// ... etc
    Ok(())
}
```

You could also write an `enum Error` which has a variant for `std::io::Error` and a variant for `simple_db::Error`, and suitable `impl From<...> for Error` blocks.

When handling a client, you *could* `.unwrap()` the function which handles the client, but do you want to quit the server if the client sends a malformed message? Perhaps you should catch the result with a `match`, and print an error to the console before moving on to the next client.
</details>

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

## Creating a thread scope

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

This exercise is based on three other exercises. Check their solutions below:

- [Simple-DB](../../exercise-solutions/simple-db)
- [Connected Mailbox](../../exercise-solutions/connected-mailbox)
- [Multithreaded mailbox](../../exercise-solutions/multi-threaded-mailbox)

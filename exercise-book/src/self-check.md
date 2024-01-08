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
8. Add support for multiple simultaneous client connections using threads. Make sure all clients read and write to the same shared queue.

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

### Solution

This exercise is based on three other exercises. Check their solutions below:

- [Simple-DB](../../exercise-solutions/simple-db)
- [Connected Mailbox](../../exercise-solutions/connected-mailbox)
- [Multithreaded mailbox](../../exercise-solutions/multi-threaded-mailbox)

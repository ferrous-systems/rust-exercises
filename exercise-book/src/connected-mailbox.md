# The Connected Mailbox
----

In this exercise we bring the SimpleDB protocol together with a TCP server in a workspace.

## After completing this exercise you are able to
* use a cargo workspace
* write docs for a crate
* 

1. Turn you project into a workspace


Read <https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html>

The workspace should contain your server and the redisish library.

     📂 mailbox-server
     ┣ 📄 Cargo.toml
     ┃
     ┣ 📂 redisish
     ┃  ┣ 📄 Cargo.toml
     ┃  ┗ ...
     ┃
     ┣ 📂 mailbox
       ┣ 📄 Cargo.toml
       ┗ ...

    $ cat mailbox-server/Cargo.toml
    [workspace]
    members = ["redisish", "mailbox"]

    $ cat mailbox-server/mailbox/Cargo.toml
    [dependencies]
    redisish = { path = "../redisish" }

1. Accept connections and implement the protocol

2.  Every connection just sends one command

3.  PUBLISH inserts into a message queue

4.  RETRIEVE retrieves a message, in a FIFO fashion

Use `.unwrap` for all error handling in the beginning.

Use `std::collections::VecDeque` for storing.

3. Do proper error handling


Implement `std::error::Error` for all your error types and start passing
around a dynamically dispatched error.

Note
----

To send and receive messages, you can either use `nc` or `telnet`.
Alternatively, you can use the client provided:
<https://github.com/ferrous-systems/teaching-material/tree/main/assignments/solutions/tcp-client>

Usage:

    $ cargo run -- "PUBLISH This is my message"
    $ cargo run -- "RETRIEVE"

The help section omits Step 4, "proper error handling" and makes the
server panic if bad data is received.

Parsing a command from a TCPStream
----

We highly recommend moving reading and parsing into its own function.

    // At the top of your file
    use std::net::{TcpListener,TcpStream};
    use std::io::prelude::*;

    fn read_command(stream: &mut TcpStream) -> redisish::Command {
        let mut read_buffer = String::new(); 
        stream.read_to_string(&mut read_buffer).unwrap(); 

        redisish::parse(&read_buffer).unwrap() 
    }

-   Allocate a read buffer (initially empty).

-   Read all data into the buffer until the client sends EOF.

-   Parse the incoming buffer

Handling the received command
----

You can copy-paste this snippet to quickly set you up reading and
matching commands.

    let command = read_command(&mut stream);
    match command {
        redisish::Command::Publish(message) => {

        }
        redisish::Command::Retrieve => {

        }
    }

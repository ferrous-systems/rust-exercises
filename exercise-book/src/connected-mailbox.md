# Connected Mailbox Exercise

In this exercise, we will take our "SimpleDB" protocol parser and turn it into a network-connected data storage service. When a user sends a "PUBLISH" we will push the data into a queue, and when the user sends a "RETRIEVE" we will pop some data off the queue (if any is available). The user will connect via TCP to port 7878.

## After completing this exercise you are able to

- write a Rust binary that uses a Rust library

- combine two Rust packages into a Cargo Workspace

- open a TCP port and perform an action when each user connects

- use I/O traits to read/write from a TCP socket

## Prerequisites

- creating and running binary crates with `cargo`

- using `match` to pattern-match on an `enum`, capturing any inner values

- using Rust's `Read` and `Write` I/O traits

- familiarity with TCP socket *listening* and *accepting*

## Tasks

1. Create an empty folder called `connected-mailbox`. Copy in the `simple-db` project from before and create a new binary crate called `tcp-server`, and put them both into a Cargo Workspace.

   ```text
   📂 connected-mailbox
   ┣ 📄 Cargo.toml 
   ┃
   ┣ 📂 simple-db 
   ┃  ┣ 📄 Cargo.toml 
   ┃  ┗ ...
   ┃
   ┗ 📂 tcp-server 
      ┣ 📄 Cargo.toml 
      ┗ ...
   ```

2. Write a basic TCP Server which can *listen* for TCP connections on `127.0.0.1:7878`. For each incoming connection, read all of the input as a string, and send it back to the client.
3. Change the TCP Server to *depend* upon the `simple-db` crate, using a relative path.
4. Change your TCP Server to *use* your `simple-db` crate to parse the input, and provide an appropriate canned response.
5. Set up a `VecDeque` and either *push* or *pop* from that queue, depending on the command you have received.

At every step, try out your program using a command-line TCP Client: you can either use `nc`, or `netcat`, or our supplied `tools/tcp-client` program.

```bash
echo "PUBLISH 1234" | nc 127.0.0.1 7878
```

The `echo` command adds a new-line character automatically. Use `echo -n` if you don't want it to add a new-line character.

## Optional Tasks:

- Run `cargo clippy` on your codebase.
- Run `cargo fmt` on your codebase.
- Wrap your `VecDeque` into a `struct Application` with a method that takes a `simple-db::Command` and returns an `Option<String>`. Write some tests for it.

## Help

### Running `nc` on Windows

Windows doesn't come with a TCP client out of the box. You have a number of options:

1. Git-for-Windows comes with `Git-Bash` - a minimal Unix emulation layer. It has Windows ports of many popular UNIX command-line utilities, including `nc`.
2. If you have WSL set up then your Linux environment has `nc` (or it is available as a package).
   You may either run the exercise in your Linux environment, too, or connect from Linux guest to your host.
3. There's a Windows-native version of [`ncat` from Nmap project that is available as a separate portable download](https://nmap.org/ncat/)
4. If you have access to a remote Linux server you can use SSH tunnelling to connect remote `nc` to a TCP server running on your local machine.
   `ssh -L 7878:<remote_host>:8888 <user>@<remote_host> -p <ssh_port>` will let you run `nc 0.0.0.0 8888` on your Linux box and talk to a locally run TCP Echo server example.
5. If you have friends that can run `nc` you can let them connect to your developer machine and play a role of your client.
   It's often possible if you share the same local network with them, but you can always rely on [`ngrok`](https://ngrok.com/docs/tcp/) or [`cloudflared`](https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/configure-tunnels/local-management/configuration-file/#supported-protocols) to expose a specific TCP port to anyone on the internet.

### Using our TCP Client

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

If you need a working example of a basic TCP Echo server, you can start with [this exercise](tcp-server.md).

### Making a Workspace

<details>
  <summary>Solution</summary>
A workspace file looks like:

```toml
[workspace]
resolver= "2"
members = ["simple-db", "tcp-server"]
```

Each member is a folder containing a Cargo package (i.e. that contains a `Cargo.toml` file).
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

### Solution

If you need it, we have provided a [complete solution](../../exercise-solutions/connected-mailbox) for this exercise.

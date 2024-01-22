# Interactive TCP Echo Server

In this exercise, we will make a simple TCP "echo" server using APIs in Rust standard Library. Here's how an interaction with it would look like from a client point of view. You connect to it using `nc`, for example:

`nc localhost 7878`

and type in one line of text. As soon as you hit enter, the server sends the line back but keeps the connection opened. You can type another line and get it back, and so on.

Here's an example interaction with the server. Notice that after typing a single line the connection is not closed and we receive the line back. All inputs and outputs should be separated by new line characters (`\n`).

```text
$ nc localhost 7878
hello
> hello
world
> world
```

(`>` denotes the text that is send back to you)

## After completing this exercise you are able to

- open a TCP port and react to each user connects

- use I/O traits to read/write from a TCP socket

- use threads to support multiple connections

## Tasks

1. Create a new binary project `tcp-server`
2. Implement a basic TCP server that listens for connections on a given port (you can use `127.0.0.1:7878` or any other port that you like).
3. Implement a loop that would read data from a `TcpStream` one line at a time. We assume that lines are separated by a `\n` character.
4. Add writing the received line back to the stream. Resolve potential borrow checker issues using standard library APIs.
5. Use Rust's `thread` API to add support for multiple connections.

Here's a bit of code to get you started:

```rust
use std::{io, net::{TcpListener, TcpStream}};

fn handle_client(mut stream: TcpStream) -> Result<(), io::Error> {
    todo!("read stream line by line, write lines back to the stream");
    // for line in stream {
    //   write line back to the to stream
    // }
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let listener = todo!("bind a listener to 127.0.0.1:7878");
    for stream in todo!("accept incoming connections") {
        // todo!("support multiple connections in parallel");
        handle_client(stream)?;
    }
    Ok(())
}
```

## Help

### Reading line by line

[Rust by Example has a chapter showing examples of reading files line by line](https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html) that chan be adapted to `TcpStream`, too.

### Solving borrow checker issues.

<details>
    <summary>At some point you may run into borrow checker issues because you essentially try writing into a stream as you read from it.</summary>

The solution is to end up with two separate owned variables that perform reading and writing respectively.

There are two general approaches to do so:

1. Simply clone the stream. `TcpStream` has a `try_clone()` method. This will not clone the stream itself: on the Operating System level there will still be a single connection. But from Rust perspective now this underlying OS resource will be represented by two distinct variables.
2. Use the fact that `Read` and `Write` traits are implemented not only for `TcpStream` but also for `&TcpStream`. For example, you can create a pair of `BufReader` and `BufWriter` by passing `&stream` as an argument.
</details>

### Troubleshooting I/O operations.

If you decide to use `BufWriter` to handle writes you may not see any text echoed back in the terminal when using `nc`. As the name applies the output is buffered, and you need to explicitly call `flush()` method for text to be send out over TCP socket.

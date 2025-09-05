<!-- markdownlint-disable MD033 -->
# Interactive TCP Echo Server

In this exercise, we will make a simple TCP "echo" server using APIs in Rust's Standard Library.

Here's how an interaction with it would look like from a client point of view. You connect to it using `nc`, for example:

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

(`>` denotes the text that is sent back to you)

## After completing this exercise you are able to

- open a TCP port and react to TCP clients connecting

- use I/O traits to read/write from a TCP socket

- use threads to support multiple connections

## Tasks

1. Create a new binary project `tcp-server`
2. Implement a basic TCP server that listens for connections on a given port (you can use `127.0.0.1:7878` or any other port that you like).
3. Implement a loop that would read data from a `TcpStream` one line at a time. We assume that lines are separated by a `\n` character.
4. Add writing the received line back to the stream. Resolve potential borrow checker issues using standard library APIs.
5. Use Rust's `thread` API to add support for multiple connections.

Here's a bit of code to get you started:

```rust ignore
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

[Rust by Example has a chapter showing examples of reading files line by line](https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html) that can be adapted to `TcpStream`, too.

### Solving borrow checker issues

<details>
    <summary>At some point you may run into borrow checker issues because you are essentially trying to write into a stream as you read from it.</summary>

The solution is to end up with two separate owned variables that perform reading and writing respectively.

There are two general approaches to do so:

1. Simply clone the stream. `TcpStream` has a `try_clone()` method. This will not clone the stream itself: on the Operating System level there will still be a single connection. But from Rust perspective now this underlying OS resource will be represented by two distinct variables.
2. Use the fact that `Read` and `Write` traits are implemented not only for `TcpStream` but also for `&TcpStream`. For example, you can create a pair of `BufReader` and `BufWriter` by passing `&stream` as an argument.

</details>

### Troubleshooting I/O operations

If you decide to use `BufWriter` to handle writes you may not see any text echoed back in the terminal when using `nc`. As the name applies the output is buffered, and you need to explicitly call `flush()` method for text to be send out over the TCP socket.

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

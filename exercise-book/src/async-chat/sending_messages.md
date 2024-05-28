## Sending Messages

Now it's time to implement the other half -- sending messages.
As a rule of thumb, only a single task should write to each `TcpStream`.
This way, we also have compartmentalised that activity and automatically serialize all outgoing messages.
So let's create a `connection_writer_loop` task which receives messages over a channel and writes them to the socket.
If Alice and Charley send two messages to Bob at the same time, Bob will see the messages in the same order as they arrive in the channel.

```rust
# extern crate tokio;
# use std::{
#     collections::hash_map::{Entry, HashMap},
#     future::Future,
# };
# 
# use tokio::{
#     io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
#     net::{tcp::OwnedWriteHalf, TcpListener, TcpStream, ToSocketAddrs},
#     sync::oneshot,
#     task,
# };
# 
# type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
use tokio::sync::mpsc; // 1

type Sender<T> = mpsc::UnboundedSender<T>; // 2
type Receiver<T> = mpsc::UnboundedReceiver<T>;

async fn connection_writer_loop(
    messages: &mut Receiver<String>,
    stream: &mut OwnedWriteHalf
) -> Result<()> {
    loop {
        let msg = messages.recv().await;
        match msg {
            Some(msg) => stream.write_all(msg.as_bytes()).await?,
            None => break,
        }
    }
    Ok(())
}
```

1. We will use `mpsc` channels from `tokio`.
2. For simplicity, we will use `unbounded` channels, and won't be discussing backpressure in this tutorial.
3. As `connection_loop` and `connection_writer_loop` share the same `TcpStream`, we need to split it into a reader and a writer:

    ```rust
    # extern crate tokio;
    # use tokio::net::TcpStream;
    # async fn connection_loop(stream: TcpStream) {
    #
    use tokio::net::tcp;
    let (reader, writer): (tcp::OwnedReadHalf, tcp::OwnedWriteHalf) = stream.into_split();
    # }
    ```

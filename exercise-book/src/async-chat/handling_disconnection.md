## Handling Disconnections

Currently, we only ever _add_ new peers to the map.
This is clearly wrong: if a peer closes connection to the chat, we should not try to send any more messages to it.

One subtlety with handling disconnection is that we can detect it either in the reader's task, or in the writer's task.
The most obvious solution here is to just remove the peer from the `peers` map in both cases, but this would be wrong.
If _both_ read and write fail, we'll remove the peer twice, but it can be the case that the peer reconnected between the two failures!
To fix this, we will only remove the peer when the write side finishes.
If the read side finishes we will notify the write side that it should stop as well.
That is, we need to add an ability to signal shutdown for the writer task.

One way to approach this is a `shutdown: Receiver<()>` channel.
There's a more minimal solution however, which makes clever use of RAII.
Closing a channel is a synchronization event, so we don't need to send a shutdown message, we can just drop the sender.
This way, we statically guarantee that we issue shutdown exactly once, even if we early return via `?` or panic.

First, let's add a shutdown channel to the `connection_loop`:

```rust
# extern crate tokio;
# use std::future::Future;
# use tokio::{
#     io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
#     net::{tcp::OwnedWriteHalf, TcpListener, TcpStream, ToSocketAddrs},
#     sync::{mpsc, oneshot},
#     task,
# };
# type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
# type Sender<T> = mpsc::UnboundedSender<T>;
# type Receiver<T> = mpsc::UnboundedReceiver<T>;
#
# async fn connection_writer_loop(
#     messages: &mut Receiver<String>,
#     stream: &mut OwnedWriteHalf,
# ) -> Result<()> {
# Ok(())
# }
#
# fn spawn_and_log_error<F>(fut: F) -> task::JoinHandle<()>
# where
#     F: Future<Output = Result<()>> + Send + 'static,
# {
#     unimplemented!()
# }
#

#[derive(Debug)]
enum Event {
    NewPeer {
        name: String,
        stream: OwnedWriteHalf,
        shutdown: oneshot::Receiver<()>,
    },
    Message {
        from: String,
        to: Vec<String>,
        msg: String,
    },
}

async fn connection_loop(broker: Sender<Event>, stream: TcpStream) -> Result<()> {
    # let (reader, writer) = stream.into_split();
    # let reader = BufReader::new(reader);
    # let mut lines = reader.lines();
    # let name: String = String::new();
    // ...
    let (_shutdown_sender, shutdown_receiver) = oneshot::channel::<()>();
    broker
        .send(Event::NewPeer {
            name: name.clone(),
            stream: writer,
            shutdown: shutdown_receiver,
        })
        .unwrap();
    // ...
#   unimplemented!()
}
```

1. To enforce that no messages are sent along the shutdown channel, we use a oneshot channel.
2. We pass the shutdown channel to the writer task.
3. In the reader, we create a `_shutdown_sender` whose only purpose is to get dropped.

In the `connection_writer_loop`, we now need to choose between shutdown and message channels.
We use the `select` macro for this purpose:

```rust
# extern crate tokio;
# use std::future::Future;
# use tokio::{
#     io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
#     net::{tcp::OwnedWriteHalf, TcpListener, TcpStream, ToSocketAddrs},
#     sync::{mpsc, oneshot},
#     task,
# };
# type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
# type Sender<T> = mpsc::UnboundedSender<T>;
# type Receiver<T> = mpsc::UnboundedReceiver<T>;
#
async fn connection_writer_loop(
    messages: &mut Receiver<String>,
    stream: &mut OwnedWriteHalf,
    mut shutdown: oneshot::Receiver<()>, // 1
) -> Result<()> {
    loop { // 2
        tokio::select! {
            msg = messages.recv() => match msg {
                Some(msg) => stream.write_all(msg.as_bytes()).await?,
                None => break,
            },
            _ = &mut shutdown => break
        }
    }
    Ok(())
}
```

1. We add shutdown channel as an argument.
2. Because of `select`, we can't use a `while let` loop, so we desugar it further into a `loop`.
3. In the shutdown case break the loop.

Another problem is that between the moment we detect disconnection in `connection_writer_loop` and the moment when we actually remove the peer from the `peers` map, new messages might be pushed into the peer's channel.
To not lose these messages completely, we'll return the messages channel back to the broker.
This also allows us to establish a useful invariant that the message channel strictly outlives the peer in the `peers` map, and makes the broker itself infallible.

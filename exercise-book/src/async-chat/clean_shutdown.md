## Clean Shutdown

One of the problems of the current implementation is that it doesn't handle graceful shutdown.
If we break from the accept loop for some reason, all in-flight tasks are just dropped on the floor.
A more correct shutdown sequence would be:

1. Stop accepting new clients
2. Deliver all pending messages
3. Exit the process

A clean shutdown in a channel based architecture is easy, although it can appear a magic trick at first.
In Rust, receiver side of a channel is closed as soon as all senders are dropped.
That is, as soon as producers exit and drop their senders, the rest of the system shuts down naturally.
In `tokio` this translates to two rules:

1. Make sure that channels form an acyclic graph.
2. Take care to wait, in the correct order, until intermediate layers of the system process pending messages.

In `a-chat`, we already have an unidirectional flow of messages: `reader -> broker -> writer`.
However, we never wait for broker and writers, which might cause some messages to get dropped.
Let's add waiting to the server:

```rust
# extern crate tokio;
# use std::{
#     collections::hash_map::{Entry, HashMap},
#     future::Future,
# };
# use tokio::{
#     io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
#     net::{tcp::OwnedWriteHalf, TcpListener, TcpStream, ToSocketAddrs},
#     sync::{mpsc, oneshot},
#     task,
# };
# type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
# type Sender<T> = mpsc::UnboundedSender<T>;
# type Receiver<T> = mpsc::UnboundedReceiver<T>;
# enum Event {
#     NewPeer {
#         name: String,
#         stream: OwnedWriteHalf,
#         shutdown: oneshot::Receiver<()>,
#     },
#     Message {
#         from: String,
#         to: Vec<String>,
#         msg: String,
#     },
# }
# async fn broker_loop(mut events: Receiver<Event>) {}
# async fn connection_loop(broker: Sender<Event>, stream: TcpStream) -> Result<()> {
#     Ok(())
# }
# fn spawn_and_log_error<F>(fut: F) -> task::JoinHandle<()>
# where
#     F: Future<Output = Result<()>> + Send + 'static,
# {
#     unimplemented!()
# }
# 
async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;

    let (broker_sender, broker_receiver) = mpsc::unbounded_channel();
    let broker = task::spawn(broker_loop(broker_receiver));

    while let Ok((stream, _socket_addr)) = listener.accept().await {
        println!("Accepting from: {}", stream.peer_addr()?);
        spawn_and_log_error(connection_loop(broker_sender.clone(), stream));
    }
    drop(broker_sender); // 1
    broker.await?; // 5
    Ok(())
}
```

Event + connection_loop:

```rust
# extern crate tokio;
# use std::{
#     collections::hash_map::{Entry, HashMap},
#     future::Future,
# };
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
#[derive(Debug)]
enum Event {
    NewPeer {
        name: String,
        stream: OwnedWriteHalf,
        shutdown: oneshot::Receiver<()>,
    },
    Message { /* unchanged */ },
}

async fn connection_loop(broker: Sender<Event>, stream: TcpStream) -> Result<()> {
    # let (reader, writer) = stream.into_split();
    # let reader = BufReader::new(reader);
    # let mut lines = reader.lines();
    # let name = String::new();
    // ...
    let (_shutdown_sender, shutdown_receiver) = oneshot::channel::<()>();
    broker
        .send(Event::NewPeer {
            name: name.clone(),
            stream: writer,
            shutdown: shutdown_receiver,
        })
        .unwrap();

    while let Ok(Some(line)) = lines.next_line().await {
        // ...
    }

    Ok(())
}
```

And to the broker:

```rust
# extern crate tokio;
# use std::{
#     collections::hash_map::{Entry, HashMap},
#     future::Future,
# };
# use tokio::{
#     io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
#     net::{tcp::OwnedWriteHalf, TcpListener, TcpStream, ToSocketAddrs},
#     sync::{mpsc, oneshot},
#     task,
# };
# type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
# type Sender<T> = mpsc::UnboundedSender<T>;
# type Receiver<T> = mpsc::UnboundedReceiver<T>;
# enum Event {
#     NewPeer {
#         name: String,
#         stream: OwnedWriteHalf,
#         shutdown: oneshot::Receiver<()>,
#     },
#     Message {
#         from: String,
#         to: Vec<String>,
#         msg: String,
#     },
# }
# async fn connection_loop(broker: Sender<Event>, stream: TcpStream) -> Result<()> {
#     Ok(())
# }
# fn spawn_and_log_error<F>(fut: F) -> task::JoinHandle<()>
# where
#     F: Future<Output = Result<()>> + Send + 'static,
# {
#     unimplemented!()
# }
# async fn connection_writer_loop(
#     messages: &mut Receiver<String>,
#     stream: &mut OwnedWriteHalf,
#     mut shutdown: oneshot::Receiver<()>,
# ) -> Result<()> {
#     Ok(())
# }
# 
async fn broker_loop(mut events: Receiver<Event>) {
    let mut peers: HashMap<String, Sender<String>> = HashMap::new();

    while let Some(event) = events.recv().await {
        match event {
            Event::Message { from, to, msg } => {
                // ...
            }
            Event::NewPeer {
                name,
                mut stream,
                shutdown,
            } => match peers.entry(name.clone()) {
                Entry::Occupied(..) => (),
                Entry::Vacant(entry) => {
                    let (client_sender, mut client_receiver) = mpsc::unbounded_channel();
                    entry.insert(client_sender);
                    spawn_and_log_error(async move {
                        connection_writer_loop(&mut client_receiver, &mut stream, shutdown).await
                    });
                }
            },
        }
    }
}
```

connection_writer_loop:

```rust
# extern crate tokio;
# use std::{
#     collections::hash_map::{Entry, HashMap},
#     future::Future,
# };
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
    mut shutdown: oneshot::Receiver<()>,
) -> Result<()> {
    loop {
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

Notice what happens with all of the channels once we exit the accept loop:

1. First, we drop the main broker's sender.
   That way when the readers are done, there's no sender for the broker's channel, and the channel closes.
2. Next, the broker exits `while let Some(event) = events.next().await` loop.
3. It's crucial that, at this stage, we drop the `peers` map.
   This drops writer's senders.
4. Now we can join all of the writers.
5. Finally, we join the broker, which also guarantees that all the writes have terminated.

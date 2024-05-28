## Clean Shutdown

One of the problems of the current implementation is that it doesn't handle graceful shutdown.
If we break from the accept loop for some reason, all in-flight tasks are just dropped on the floor.

We will intercept `Ctrl-C`.

A more correct shutdown sequence would be:

1. Stop accepting new clients
2. Notify the readers we're not accepting new messages
3. Deliver all pending messages
4. Exit the process

A clean shutdown in a channel based architecture is easy, although it can appear a magic trick at first.
In Rust, receiver side of a channel is closed as soon as all senders are dropped.
That is, as soon as producers exit and drop their senders, the rest of the system shuts down naturally.
In `tokio` this translates to two rules:

1. Make sure that channels form an acyclic graph.
2. Take care to wait, in the correct order, until intermediate layers of the system process pending messages.

In `a-chat`, we already have an unidirectional flow of messages: `reader -> broker -> writer`.
However, we never wait for broker and writers, which might cause some messages to get dropped.

We also need to notify all readers that we are going to stop accepting messages. Here, we use `tokio::sync::Notify`.

Let's first add the notification feature to the readers.
We have to start using `select!` here to work 
```rust,ignore
async fn connection_loop(broker: Sender<Event>, stream: TcpStream, shutdown: Arc<Notify>) -> Result<()> {
    // ...
    loop {
        tokio::select! {
            Ok(Some(line)) = lines.next_line() => {
                let (dest, msg) = match line.split_once(':') {

                    None => continue,
                    Some((dest, msg)) => (dest, msg.trim()),
                };
                let dest: Vec<String> = dest
                    .split(',')
                    .map(|name| name.trim().to_string())
                    .collect();
                let msg: String = msg.trim().to_string();
        
                broker
                    .send(Event::Message {
                        from: name.clone(),
                        to: dest,
                        msg,
                    })
                    .unwrap();
            },
            _ = shutdown.notified() => break,
        }
    }
}
```

Let's add Ctrl-C handling and waiting to the server.

```rust,ignore
# extern crate tokio;
# use std::{
#     collections::hash_map::{Entry, HashMap},
#     future::Future,
# };
# use tokio::{
#     io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
#     net::{tcp::OwnedWriteHalf, TcpListener, TcpStream, ToSocketAddrs},
#     sync::{mpsc, oneshot, Notify},
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
    let shutdown_notification = Arc::new(Notify::new());

    loop {
        tokio::select!{
            Ok((stream, _socket_addr)) = listener.accept() => {
                println!("Accepting from: {}", stream.peer_addr()?);
                spawn_and_log_error(connection_loop(broker_sender.clone(), stream, shutdown_notification.clone()));
            },
            _ = tokio::signal::ctrl_c() => break,
        }
    }
    println!("Shutting down server!");
    shutdown_notification.notify_waiters(); // 1
    drop(broker_sender); // 2
    broker.await?; // 5
    Ok(())
}
```

And to the broker:

```rust,ignore
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

    loop {
        let event = match events.recv().await {
            Some(event) => event,
            None => break,
        };        
        match event {
            Event::Message { from, to, msg } => {
                // ...
            }
            Event::NewPeer {
                name,
                mut stream,
            } => match peers.entry(name.clone()) {
                Entry::Occupied(..) => (),
                Entry::Vacant(entry) => {
                    let (client_sender, mut client_receiver) = mpsc::unbounded_channel();
                    entry.insert(client_sender);
                    spawn_and_log_error(async move {
                        connection_writer_loop(&mut client_receiver, &mut stream).await
                    });
                }
            },
        }
    }

    drop(peers) //4
}
```


Notice what happens with all of the channels once we exit the accept loop:

1. We notify all readers to stop accepting messages.
2. We drop the main broker's sender.
   That way when the readers are done, there's no sender for the broker's channel, and the channel closes.
3. Next, the broker exits `while let Some(event) = events.next().await` loop.
3. It's crucial that, at this stage, we drop the `peers` map.
   This drops writer's senders.
4. Tokio will automatically wait for all finishing futures
5. Finally, we join the broker, which also guarantees that all the writes have terminated.

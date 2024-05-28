
## A broker as a connection point

So how do we make sure that messages read in `connection_loop` flow into the relevant `connection_writer_loop`?
We should somehow maintain a `peers: HashMap<String, Sender<String>>` map which allows a client to find destination channels.
However, this map would be a bit of shared mutable state, so we'll have to wrap an `RwLock` over it and answer tough questions of what should happen if the client joins at the same moment as it receives a message.

One trick to make reasoning about state simpler is by taking inspiration from the actor model.
We can create a dedicated broker task which owns the `peers` map and communicates with other tasks using channels.
The broker reacts on events and appropriately informs the peers.
By hiding peer handling inside such an "actor" task, we remove the need for mutexes and also make the serialization point explicit.
The order of events "Bob sends message to Alice" and "Alice joins" is determined by the order of the corresponding events in the broker's event queue.

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
use std::collections::hash_map::{Entry, HashMap};

#[derive(Debug)]
enum Event { // 1
    NewPeer {
        name: String,
        stream: OwnedWriteHalf,
    },
    Message {
        from: String,
        to: Vec<String>,
        msg: String,
    },
}

async fn broker_loop(mut events: Receiver<Event>) {
    let mut peers: HashMap<String, Sender<String>> = HashMap::new(); // 2

    loop {
        let event = match events.recv().await {
            Some(event) => event,
            None => break,
        };

        match event {
            Event::Message { from, to, msg } => { // 3
                for addr in to {
                    if let Some(peer) = peers.get_mut(&addr) {
                        let msg = format!("from {from}: {msg}\n");
                        peer.send(msg).unwrap();
                    }
                }
            }
            Event::NewPeer { name, mut stream } => match peers.entry(name.clone()) {
                Entry::Occupied(..) => (),
                Entry::Vacant(entry) => {
                    let (client_sender, mut client_receiver) = mpsc::unbounded_channel();
                    entry.insert(client_sender); // 4
                    spawn_and_log_error(async move {
                        connection_writer_loop(&mut client_receiver, &mut stream).await
                    }); // 5
                }
            },
        }
    }
}
```

1. The broker task should handle two types of events: a message or an arrival of a new peer.
2. The internal state of the broker is a `HashMap`.
   Note how we don't need a `Mutex` here and can confidently say, at each iteration of the broker's loop, what is the current set of peers.
3. To handle a message, we send it over a channel to each destination.
4. To handle a new peer, we first register it in the peer's map ...
5. ... and then spawn a dedicated task to actually write the messages to the socket.

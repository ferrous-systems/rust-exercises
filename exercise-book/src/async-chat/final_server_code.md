# Final Server Code

The final code looks like this:

```rust
# extern crate tokio;
use std::{
    collections::hash_map::{Entry, HashMap},
    future::Future,
};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{tcp::OwnedWriteHalf, TcpListener, TcpStream, ToSocketAddrs},
    sync::{mpsc, oneshot},
    task,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

// main
async fn run() -> Result<()> {
    accept_loop("127.0.0.1:8080").await
}

async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;

    let (broker_sender, broker_receiver) = mpsc::unbounded_channel();
    let broker = task::spawn(broker_loop(broker_receiver));

    while let Ok((stream, _socket_addr)) = listener.accept().await {
        println!("Accepting from: {}", stream.peer_addr()?);
        spawn_and_log_error(connection_loop(broker_sender.clone(), stream));
    }
    drop(broker_sender);
    broker.await?;
    Ok(())
}

async fn connection_loop(broker: Sender<Event>, stream: TcpStream) -> Result<()> {
    let (reader, writer) = stream.into_split();
    let reader = BufReader::new(reader);
    let mut lines = reader.lines();

    let name = match lines.next_line().await {
        Ok(Some(line)) => line,
        Ok(None) => return Err("peer disconnected immediately".into()),
        Err(e) => return Err(Box::new(e)),
    };

    println!("user {} connected", name);

    let (_shutdown_sender, shutdown_receiver) = oneshot::channel::<()>();
    broker
        .send(Event::NewPeer {
            name: name.clone(),
            stream: writer,
            shutdown: shutdown_receiver,
        })
        .unwrap();

    while let Ok(Some(line)) = lines.next_line().await {
        let (dest, msg) = match line.find(':') {
            None => continue,
            Some(idx) => (&line[..idx], line[idx + 1..].trim()),
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
    }

    Ok(())
}

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

async fn broker_loop(mut events: Receiver<Event>) {
    let (disconnect_sender, mut disconnect_receiver) =
        mpsc::unbounded_channel::<(String, Receiver<String>)>(); // 1
    let mut peers: HashMap<String, Sender<String>> = HashMap::new();

    loop {
        let event = tokio::select! {
            event = events.recv() => match event {
                None => break, // 2
                Some(event) => event,
            },
            disconnect = disconnect_receiver.recv() => {
                let (name, _pending_messages) = disconnect.unwrap(); // 3
                assert!(peers.remove(&name).is_some());
                continue;
            },
        };
        match event {
            Event::Message { from, to, msg } => {
                for addr in to {
                    if let Some(peer) = peers.get_mut(&addr) {
                        let msg = format!("from {}: {}\n", from, msg);
                        peer.send(msg).unwrap(); // 6
                    }
                }
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
                    let disconnect_sender = disconnect_sender.clone();
                    spawn_and_log_error(async move {
                        let res =
                            connection_writer_loop(&mut client_receiver, &mut stream, shutdown)
                                .await;
                        disconnect_sender.send((name, client_receiver)).unwrap(); // 4
                        res
                    });
                }
            },
        }
    }
    drop(peers); // 5
    drop(disconnect_sender); // 6
    while let Some((_name, _pending_messages)) = disconnect_receiver.recv().await {}
}

fn spawn_and_log_error<F>(fut: F) -> task::JoinHandle<()>
where
    F: Future<Output = Result<()>> + Send + 'static,
{
    task::spawn(async move {
        if let Err(e) = fut.await {
            eprintln!("{}", e)
        }
    })
}
```

1. In the broker, we create a channel to reap disconnected peers and their undelivered messages.
2. The broker's main loop exits when the input events channel is exhausted (that is, when all readers exit).
3. Because broker itself holds a `disconnect_sender`, we know that the disconnections channel can't be fully drained in the main loop.
4. We send peer's name and pending messages to the disconnections channel in both the happy and the not-so-happy path.
   Again, we can safely unwrap because the broker outlives writers.
5. We drop `peers` map to close writers' messages channel and shut down the writers for sure.
   It is not strictly necessary in the current setup, where the broker waits for readers' shutdown anyway.
   However, if we add a server-initiated shutdown (for example, kbd:[ctrl+c] handling), this will be a way for the broker to shutdown the writers.
6. Finally, we close and drain the disconnections channel.

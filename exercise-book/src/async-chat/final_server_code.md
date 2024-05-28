# Final Server Code

The final code looks like this:

```rust
use std::{
    collections::hash_map::{Entry, HashMap},
    future::Future,
    sync::Arc,
};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{tcp::OwnedWriteHalf, TcpListener, TcpStream, ToSocketAddrs},
    sync::{mpsc, oneshot, Notify},
    task,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

#[tokio::main]
pub(crate) async fn main() -> Result<()> {
    accept_loop("127.0.0.1:8080").await
}

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
    println!("Shutting down!");
    shutdown_notification.notify_waiters();
    drop(broker_sender);
    broker.await?;
    Ok(())
}

async fn connection_loop(broker: Sender<Event>, stream: TcpStream, shutdown: Arc<Notify>) -> Result<()> {
    let (reader, writer) = stream.into_split();
    let reader = BufReader::new(reader);
    let mut lines = reader.lines();
    let (shutdown_sender, shutdown_receiver) = oneshot::channel::<()>();

    let name = match lines.next_line().await {
        Ok(Some(line)) => line,
        Ok(None) => return Err("peer disconnected immediately".into()),
        Err(e) => return Err(Box::new(e)),
    };

    println!("user {} connected", name);

    broker
        .send(Event::NewPeer {
            name: name.clone(),
            stream: writer,
            shutdown: shutdown_receiver,
        })
        .unwrap();
    
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
    println!("Closing connection loop!");
    drop(shutdown_sender);

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

    println!("Closing connection_writer loop!");

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
        mpsc::unbounded_channel::<(String, Receiver<String>)>();
    let mut peers: HashMap<String, Sender<String>> = HashMap::new();

    loop {
        let event = tokio::select! {
            event = events.recv() => match event {
                None => break,
                Some(event) => event,
            },
            disconnect = disconnect_receiver.recv() => {
                let (name, _pending_messages) = disconnect.unwrap();
                assert!(peers.remove(&name).is_some());
                println!("user {} disconnected", name);
                continue;
            },
        };
        match event {
            Event::Message { from, to, msg } => {
                for addr in to {
                    if let Some(peer) = peers.get_mut(&addr) {
                        let msg = format!("from {}: {}\n", from, msg);
                        peer.send(msg).unwrap();
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
                        println!("user {} disconnected", name);
                        disconnect_sender.send((name, client_receiver)).unwrap();
                        res
                    });
                }
            },
        }
    }
    drop(peers);
    drop(disconnect_sender);
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

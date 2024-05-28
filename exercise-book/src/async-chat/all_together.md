## Gluing all together

At this point, we only need to start the broker to get a fully-functioning (in the happy case!) chat.

Scroll past the example find a list of all changes.

```rust,ignore
# extern crate tokio;
use std::{
    collections::hash_map::{Entry, HashMap},
    future::Future,
};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{tcp::OwnedWriteHalf, TcpListener, TcpStream, ToSocketAddrs},
    sync::mpsc,
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

    let (broker_sender, broker_receiver) = mpsc::unbounded_channel(); // 1
    let _broker = task::spawn(broker_loop(broker_receiver));

    while let Ok((stream, _socket_addr)) = listener.accept().await {
        println!("Accepting from: {}", stream.peer_addr()?);
        spawn_and_log_error(connection_loop(broker_sender.clone(), stream));
    }
    Ok(())
}

async fn connection_loop(broker: Sender<Event>, stream: TcpStream) -> Result<()> { // 2
    let (reader, writer) = stream.into_split(); // 3
    let reader = BufReader::new(reader);
    let mut lines = reader.lines();

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
        })
        .unwrap(); // 5

    loop {
        if let Some(line) = lines.next_line().await? {
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
                .send(Event::Message { // 4
                    from: name.clone(),
                    to: dest,
                    msg,
                })
                .unwrap();
        } else {
            break;
        }
    }

    Ok(())
}

async fn connection_writer_loop(
    messages: &mut Receiver<String>,
    stream: &mut OwnedWriteHalf // 3
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

#[derive(Debug)]
enum Event {
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
    let mut peers: HashMap<String, Sender<String>> = HashMap::new();

    loop {
        let event = match events.recv().await {
            Some(event) => event,
            None => break,
        };
        match event {
            Event::Message { from, to, msg } => {
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
                    entry.insert(client_sender);
                    spawn_and_log_error(async move {
                        connection_writer_loop(&mut client_receiver, &mut stream).await
                    });
                }
            },
        }
    }
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

1. Inside the `accept_loop`, we create the broker's channel and `task`.
2. We need the connection_loop to accept a handle to the broker.
3. Inside `connection_loop`, we need to split the `TcpStream`, to be able to share it with the `connection_writer_loop`.
4. On login, we notify the broker.
   Note that we `.unwrap` on send: broker should outlive all the clients and if that's not the case the broker probably panicked, so we can escalate the panic as well.
5. Similarly, we forward parsed messages to the broker, assuming that it is alive.

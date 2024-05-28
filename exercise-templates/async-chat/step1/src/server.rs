use std::{
    collections::hash_map::{Entry, HashMap},
    future::Future,
};

use tokio::sync::{mpsc, oneshot};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::tcp::OwnedWriteHalf,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    task,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

#[tokio::main]
pub(crate) async fn main() -> Result<()> {
    accept_loop("127.0.0.1:8080").await
}

async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> { // 1
    let listener = TcpListener::bind(addr).await?; // 2

    loop { // 3
        let (stream, _) = listener.accept().await?;
        // TODO
    }

    Ok(())
}

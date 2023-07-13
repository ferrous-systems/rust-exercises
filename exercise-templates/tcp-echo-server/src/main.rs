use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

const DEFAULT_TIMEOUT: Option<Duration> = Some(Duration::from_millis(1000));

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    // accept connections and process them one at a time
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Got client {:?}", stream.peer_addr());
                if let Err(e) = handle_client(stream) {
                    println!("Error handling client: {:?}", e);
                }
            }
            Err(e) => {
                println!("Error connecting: {:?}", e);
            }
        }
    }
    Ok(())
}

/// Process a single connection from a single client.
///
/// Drops the stream when it has finished.
fn handle_client(mut stream: TcpStream) -> Result<(), std::io::Error> {
    stream.set_read_timeout(DEFAULT_TIMEOUT)?;
    stream.set_write_timeout(DEFAULT_TIMEOUT)?;

    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;
    println!("Received: {:?}", buffer);
    writeln!(stream, "Thank you for {buffer:?}!")?;
    Ok(())
}

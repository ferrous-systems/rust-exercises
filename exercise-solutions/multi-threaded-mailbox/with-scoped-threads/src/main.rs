use std::collections::VecDeque;
use std::io::{self, prelude::*};
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::time::Duration;

const DEFAULT_TIMEOUT: Option<Duration> = Some(Duration::from_millis(1000));

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    let locked_storage = Mutex::new(VecDeque::new());

    std::thread::scope(|s| {
        // accept connections and process them in parallel
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Got client {:?}", stream.peer_addr());
                    s.spawn(|| {
                        if let Err(e) = handle_client(stream, &locked_storage) {
                            println!("Error handling client: {:?}", e);
                        }
                    });
                }
                Err(e) => {
                    println!("Error connecting: {:?}", e);
                }
            }
        }
    });

    Ok(())
}

/// Process a single connection from a single client.
///
/// Drops the stream when it has finished.
fn handle_client(
    mut stream: TcpStream,
    locked_storage: &Mutex<VecDeque<String>>,
) -> io::Result<()> {
    stream.set_read_timeout(DEFAULT_TIMEOUT)?;
    stream.set_write_timeout(DEFAULT_TIMEOUT)?;

    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;
    println!("Received: {:?}", buffer);

    let command = match simple_db::parse(&buffer) {
        Ok(s) => s,
        Err(e) => {
            println!("Error parsing command: {:?}", e);
            writeln!(stream, "Error: {}!", e)?;
            return Ok(());
        }
    };

    println!("Got command {:?}", command);

    match command {
        simple_db::Command::Publish(message) => {
            let mut storage = locked_storage.lock().unwrap();
            storage.push_back(message);
            // Drop the lock before talking to the network
            drop(storage);
            writeln!(stream, "OK")?;
        }
        simple_db::Command::Retrieve => {
            let mut storage = locked_storage.lock().unwrap();
            let value = storage.pop_front();
            // Drop the lock before talking to the network
            drop(storage);
            match value {
                Some(message) => writeln!(stream, "Got: {:?}", message)?,
                None => writeln!(stream, "Error: Queue empty!")?,
            }
        }
    }
    Ok(())
}

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> Result<(), std::io::Error> {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;
    println!("Received: {:?}", buffer);
    writeln!(stream, "Thank you for {buffer:?}!")?;
    Ok(())
}

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

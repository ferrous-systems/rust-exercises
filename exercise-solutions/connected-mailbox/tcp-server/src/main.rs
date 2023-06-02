use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

#[derive(Debug)]
enum ServerError {
    ParseError(simple_db::Error),
    IoError(std::io::Error),
}

impl From<simple_db::Error> for ServerError {
    fn from(e: simple_db::Error) -> ServerError {
        ServerError::ParseError(e)
    }
}

impl From<std::io::Error> for ServerError {
    fn from(e: std::io::Error) -> ServerError {
        ServerError::IoError(e)
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    let mut storage = VecDeque::new();

    // accept connections and process them one at a time
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Got client {:?}", stream.peer_addr());
                if let Err(e) = handle_client(stream, &mut storage) {
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

fn handle_client(mut stream: TcpStream, storage: &mut VecDeque<String>) -> Result<(), ServerError> {
    let command = read_command(&mut stream)?;

    println!("Got command {:?}", command);

    match command {
        simple_db::Command::Publish(message) => {
            storage.push_back(message);
            writeln!(stream, "Message published OK.")?;
        }
        simple_db::Command::Retrieve => {
            let data = storage.pop_front();

            match data {
                Some(message) => writeln!(stream, "{}", message)?,
                None => writeln!(stream, "No message in inbox!")?,
            }
        }
    }
    Ok(())
}

fn read_command(stream: &mut TcpStream) -> Result<simple_db::Command, ServerError> {
    let mut read_buffer = String::new();
    let mut buffered_stream = BufReader::new(stream);
    buffered_stream.read_line(&mut read_buffer)?;
    let command = simple_db::parse(&read_buffer)?;
    Ok(command)
}

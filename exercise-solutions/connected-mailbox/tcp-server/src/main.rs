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
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let mut storage = VecDeque::new();

    for connection in listener.incoming() {
        let stream = match connection {
            Ok(stream) => stream,
            Err(e) => {
                println!("Error occurred: {:?}", e);
                continue;
            }
        };

        let res = handle_client(stream, &mut storage);

        if let Err(e) = res {
            println!("Error occurred: {:?}", e);
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream, storage: &mut VecDeque<String>) -> Result<(), ServerError> {
    let command = read_command(&mut stream)?;

    match command {
        simple_db::Command::Publish(message) => {
            storage.push_back(message);
        }
        simple_db::Command::Retrieve => {
            let data = storage.pop_front();

            match data {
                Some(message) => write!(stream, "{}", message)?,
                None => write!(stream, "No message in inbox!\n")?,
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

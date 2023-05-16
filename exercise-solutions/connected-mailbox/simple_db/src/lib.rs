use std::fmt;

#[derive(Eq, PartialEq, Debug)]
pub enum Command {
    Publish(String),
    Retrieve,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Error {
    TrailingData,
    IncompleteMessage,
    EmptyMessage,
    UnknownCommand,
    UnknownError,
    UnexpectedPayload,
    MissingPayload,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing is command: {:?}!", self)
    }
}

impl std::error::Error for Error {}

pub fn parse(input: &str) -> Result<Command, Error> {
    match input.split_once('\n') {
        Some((_message, trailing_data)) => {
            if trailing_data.len() != 0 {
                return Err(Error::TrailingData);
            }
        }
        None => return Err(Error::IncompleteMessage),
    }

    let mut substrings = input.splitn(2, ' ');

    if let Some(command) = substrings.next() {
        match command.trim() {
            "RETRIEVE" => {
                if substrings.next().is_none() {
                    Ok(Command::Retrieve)
                } else {
                    Err(Error::UnexpectedPayload)
                }
            }
            "PUBLISH" => {
                if let Some(payload) = substrings.next() {
                    Ok(Command::Publish(String::from(payload.trim())))
                } else {
                    Err(Error::MissingPayload)
                }
            }
            "" => Err(Error::EmptyMessage),
            _ => Err(Error::UnknownCommand),
        }
    } else {
        Err(Error::UnknownError)
    }
}

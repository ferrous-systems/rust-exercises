//! Simple DB - a parser for a simple database protocol

/// The kinds of commands our protocol supports
#[derive(Eq, PartialEq, Debug)]
pub enum Command {
    /// Publishes a new message into the database.
    ///
    /// `PUBLISH <message>\n`
    Publish(String),
    /// Retrieves a message from the database.
    ///
    /// `RETRIEVE\n`
    Retrieve,
}

/// The ways in which this module can fail
#[derive(Eq, PartialEq, Debug, thiserror::Error)]
pub enum Error {
    #[error("Unexpected data after the final newline")]
    TrailingData,
    #[error("Not enough data in command (missing newline?)")]
    IncompleteMessage,
    #[error("Input was empty")]
    EmptyMessage,
    #[error("Unknown command string")]
    UnknownCommand,
    #[error("Valid command but unexpected payload")]
    UnexpectedPayload,
    #[error("Valid command but missing payload")]
    MissingPayload,
}

pub fn parse(input: &str) -> Result<Command, Error> {
    match input.split_once('\n') {
        Some((_message, trailing_data)) => {
            if !trailing_data.is_empty() {
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
        Err(Error::IncompleteMessage)
    }
}

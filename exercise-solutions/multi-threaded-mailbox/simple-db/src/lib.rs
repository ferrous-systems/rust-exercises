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
    #[error("Found a newline that wasn't at the end")]
    UnexpectedNewline,
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

/// Parse a command-string into `Command`, or produce an `Error` if it could not
/// be parsed.
pub fn parse(input: &str) -> Result<Command, Error> {
    let Some(message) = input.strip_suffix('\n') else {
        return Err(Error::IncompleteMessage);
    };

    if message.contains('\n') {
        return Err(Error::UnexpectedNewline);
    }

    if let Some(payload) = message.strip_prefix("PUBLISH ") {
        Ok(Command::Publish(String::from(payload)))
    } else if message == "PUBLISH" {
        Err(Error::MissingPayload)
    } else if message == "RETRIEVE" {
        Ok(Command::Retrieve)
    } else if let Some(_payload) = message.strip_prefix("RETRIEVE ") {
        Err(Error::UnexpectedPayload)
    } else if message == "" {
        Err(Error::EmptyMessage)
    } else {
        Err(Error::UnknownCommand)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests placement of \n
    #[test]
    fn test_missing_nl() {
        let line = "RETRIEVE";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::IncompleteMessage);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_trailing_data() {
        let line = "PUBLISH The message\n is wrong \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnexpectedNewline);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_string() {
        let line = "";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::IncompleteMessage);
        assert_eq!(result, expected);
    }

    // Tests for empty messages and unknown commands

    #[test]
    fn test_only_nl() {
        let line = "\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::EmptyMessage);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_unknown_command() {
        let line = "SERVE \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnknownCommand);
        assert_eq!(result, expected);
    }

    // Tests correct formatting of RETRIEVE command

    #[test]
    fn test_retrieve_w_whitespace() {
        let line = "RETRIEVE \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnexpectedPayload);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_retrieve_payload() {
        let line = "RETRIEVE this has a payload\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnexpectedPayload);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_retrieve() {
        let line = "RETRIEVE\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Ok(Command::Retrieve);
        assert_eq!(result, expected);
    }

    // Tests correct formatting of PUBLISH command

    #[test]
    fn test_publish() {
        let line = "PUBLISH TestMessage\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Ok(Command::Publish("TestMessage".into()));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_publish() {
        let line = "PUBLISH \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Ok(Command::Publish("".into()));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_missing_payload() {
        let line = "PUBLISH\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::MissingPayload);
        assert_eq!(result, expected);
    }
}

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

pub fn parse(input: &str) -> Result<Command, Error> {
    match input.split_once('\n') {
        Some((_, data)) => {
            if data.len() != 0 {
                return Err(Error::TrailingData);
            }
        }
        None => return Err(Error::IncompleteMessage),
    }

    let mut substrings = input.splitn(2, ' ');

    if let Some(command) = substrings.next() {
        match command.trim() {
            "" => Err(Error::EmptyMessage),
            _ => Err(Error::UnknownCommand),
        }
    } else {
        Err(Error::UnknownError)
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
        let expected = Err(Error::TrailingData);
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
}

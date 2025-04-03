#[derive(Eq, PartialEq, Debug)]
pub enum Command {
    Publish(String),
    Retrieve,
    Command, // introduced only temporarily
}

#[derive(Eq, PartialEq, Debug)]
pub enum Error {
    UnexpectedNewline,
    IncompleteMessage,
    EmptyMessage,
    UnknownCommand,
    UnexpectedPayload,
    MissingPayload,
}

pub fn parse(input: &str) -> Result<Command, Error> {
    let Some(message) = input.strip_suffix('\n') else {
        return Err(Error::IncompleteMessage);
    };

    if message.contains('\n') {
        return Err(Error::UnexpectedNewline);
    }

    Ok(Command::Command)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trailing_data() {
        let line = "PUBLISH The message\n is wrong \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnexpectedNewline);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_missing_nl() {
        let line = "PUBLISH";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::IncompleteMessage);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_correct_nl() {
        let line = "PUBLISH \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Ok(Command::Command);
        assert_eq!(result, expected);
    }
}

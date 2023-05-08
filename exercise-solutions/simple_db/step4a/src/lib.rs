#[derive(Eq, PartialEq, Debug)]
pub enum Command {
    Publish(String),
    Retrieve,
    Command, // introduced only temporarely
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
                Err(Error::TrailingData)
            } else {
                Ok(Command::Command)
            }
        }
        None => Err(Error::IncompleteMessage),
    }
}

#[test]
fn test_trailing_data() {
    let line = "PUBLISH The message\n is wrong \n";
    let result: Result<Command, Error> = parse(line);
    let expected = Err(Error::TrailingData);
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
}

#[derive(Eq, PartialEq, Debug)]
pub enum Command {
    Publish(String),
    Retrieve,
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

// Tests go here!

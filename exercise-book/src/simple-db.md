# SimpleDB Exercise

In this exercise, we will implement a toy protocol parser for a simple protocol for databank queries. We call it simpleDB. The protocol has two commands, one of them can be sent with a payload of additional data. Your parser parses the incoming data strings, makes sure the commands are formatted correctly and returns errors for the different ways the formatting can go wrong.

## After completing this exercise you are able to

- write a simple Rust library from scratch

- interact with borrowed and owned memory, especially how to take ownership

- handle complex cases using the `match` and `if let` syntax

- create a safe protocol parser in Rust manually


## Prerequisites

- basic pattern matching with `match`

- control flow with if/else

- familiarity with `Result<T, E>`, `Option<T>`

## Tasks

1. Create a library project called `simple_db`.
2. Implement appropriate data structures for `Command` and `Error`.
3. Read the documentation for `str` (primitive), especially [`split_once()`](https://doc.rust-lang.org/std/primitive.str.html#method.split_once), [`splitn()`](https://doc.rust-lang.org/std/primitive.str.html#method.splitn) and [`trim()`](https://doc.rust-lang.org/std/primitive.str.html#method.trim). Pay attention to their return type. Use the result value of `split_once()` and `splitn()` to guide your logic. The Step-by-Step-Solution contains a proposal.
4. Implement the following function so that it implements the protocol specifications to parse the messages. Use the provided tests to help you with the case handling.

```rust, ignore
    fn parse(input: &str) -> Result<Command, Error> {
        todo!()
    }
```

The Step-by-Step-Solution contains steps 4a-e that explain a possible way to handle the cases in detail.

### Optional Tasks:

- Run `clippy` on your codebase.
- Run `rustfmt` on your codebase.

If you need it, we have provided [solutions](../../exercise-solutions/simple_db/) for every step for this exercise.

### Protocol Specification

The protocol has two commands that are sent as messages in the following
form:

- `PUBLISH <payload>\n`

- `RETRIEVE\n`

With the additional properties:

1. The payload cannot contain newlines.

2. A missing newline at the end of the command is an error.

3. Data after the first newline is an error.

4. Empty payloads are allowed. In this case, the command is
    `PUBLISH \n`.

Violations against the form of the messages and the properties are
handled with the following error codes:

- TrailingData (more than one newline)

- IncompleteMessage (no newline)

- EmptyMessage (empty string instead of a command)

- UnknownCommand (string is not empty, but neither PUBLISH nor
    RECEIVE)

- UnexpectedPayload (message contains a payload, when it should not)

- MissingPayload (message is missing a payload)

- UnknownError (message does not contain a string)

### Testing

Below are the tests your protocol parser needs to pass. You can copy them to the bottom of your `lib.rs`.

```rust, ignore
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

        #[test]
        fn test_empty_string() {
            let line = "";
            let result = parse(line);
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
```

# SimpleDB Exercise

In this exercise, we will implement a toy protocol parser for a simple protocol for databank queries. We call it simpleDB. The protocol has two commands, one of them can be sent with a payload of additional data. Your parser parses the incoming data strings, makes sure the commands are formatted correctly and returns errors for the different ways the formatting can go wrong.

## After completing this exercise you will be able to

- write a simple Rust library from scratch

- interact with borrowed and owned memory, especially how to take ownership

- handle complex cases using the `match` and `if let` syntax

- create a safe protocol parser in Rust manually

## Prerequisites

- basic pattern matching with `match`

- control flow with if/else

- familiarity with `Result<T, E>`, `Option<T>`

## Tasks

1. Create a library project called `simple-db`.
2. Implement appropriate data structures for `Command` and `Error`.
3. Read the documentation for [`str`](https://doc.rust-lang.org/std/primitive.str.html), especially [`strip_prefix()`](https://doc.rust-lang.org/std/primitive.str.html#method.strip_prefix) and [`strip_suffix()`](https://doc.rust-lang.org/std/primitive.str.html#method.strip_suffix). Pay attention to their return type.
4. Implement the following function so that it implements the protocol specifications to parse the messages. Use the provided tests to help you with the case handling.

```rust, ignore
pub fn parse(input: &str) -> Result<Command, Error> {
    todo!()
}
```

The Step-by-Step-Solution contains steps 4a-c that explain a possible way to handle the cases in detail.

### Optional Tasks:

- Run `clippy` on your codebase.
- Run `rustfmt` on your codebase.

If you need it, we have provided [solutions](../../exercise-solutions/simple-db/) for every step for this exercise.

### Protocol Specification

The protocol has two commands that are sent as messages in the following
form:

- `PUBLISH <payload>\n`

- `RETRIEVE\n`

With the additional properties:

1. The payload cannot contain newlines.

2. A missing newline at the end of the command is an error.

3. A newline other than at the end of the command is an error.

4. Sending a `PUBLISH` with an empty payload is allowed. In this case, the command `PUBLISH \n` 
will publish an empty payload.

*Note*: depending on the order in which the rules are implement, you may obtain different behaviours.

Issues with the format (or other properties) of the messages are
handled with the following error codes:

- `UnexpectedNewline` (a newline not at the end of the line)

- `IncompleteMessage` (no newline at the end)

- `EmptyMessage` (empty string instead of a command)

- `UnknownCommand` (string is not empty, but neither `PUBLISH` nor
    `RECEIVE`)

- `UnexpectedPayload` (message contains a payload, when it should not)

- `MissingPayload` (message is missing a payload)

### Testing

Below are the tests your protocol parser needs to pass. You can copy them to the bottom of your `lib.rs`.

```rust, ignore
{{#include ../../exercise-solutions/simple-db/step4c/src/lib.rs:41:138}}
```

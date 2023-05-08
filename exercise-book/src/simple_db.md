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

## Knowledge

This section explains concepts necessary to solve this exercise.

In general, we also recommend to use the Rust documentation to figure
out things you are missing to familiarize yourself with it. If you ever
feel completely stuck or that you haven’t understood something, please
hail the trainers quickly.

### Derives

`#[derive(PartialEq, Eq)]`

  This enables comparison between 2 instances of the type, by comparing every field/variant. This enables the `assert_eq!` macro, which relies on equality being defined. `Eq` for total equality isn’t strictly necessary for this example, but it is good practice to derive it if it applies.

`#[derive(Debug)]`

This enables automatic debug output for the type. The `assert_eq!`macro requires this for testing.


### Control flow and pattern matching, returning values

This exercise involves handling a number of cases. You are already
familiar with `if /else` and a basic form of `match`. Here, we’ll
introduce you to `if let`.

```rust, ignore
    if let Some(payload) = substrings.next() {
        // execute if the above statement is true
    }
```

`if let` assigns and evaluates in one line. A typical use is to assign
the returned `Option(T)` from a method to `Some(T)`. The statement
yields true, if `Some(T)` is returned, false if `None` is returned.

#### When to use what?

`if let` is used if you have to decide between two cases, where the
second case is usually of lesser meaning for the program’s execution.

`match` can be used to handle more fine grained and complex pattern
matching, especially when there are several, equally ranked
possibilities. The match arms have to include a catch all `_ =>` arm,
for every possible case that is not explicitly spelled out. The order
of the match arms matter: The catch all branch needs to be last,
otherwise, it catches all…

#### Returning Values from branches and match arms

- all match arms always need to return the same type, or none can
    return a value.

- For `if let/else` or `if/else`: If there is no explicit `else`
    branch, it implicitly returns `()`. If you run into trouble because
    you need a return type, but don’t need the else condition, `return`
    statements can help.

## Step by Step Solution

### Step 1: Creating a library project with cargo

Create a new Cargo project, check the build and the test setup:

<details>
  <summary>Solution</summary>

```console
cargo new --lib simple_db 
cd simple_db 
cargo build 
cargo test
```

</details>

### Step 2: Appropriate data structures

Define two enums, one is called `Command` and one is called `Error`. `Command` has 2 variants for the two possible commands. `Publish` carries data (the message), `Retrieve` does not. `Error` is just a list of error *kinds*. Use `#[derive(Eq,PartialEq,Debug)]` for both `enums`.

<details>
  <summary>Solution</summary>

```rust, ignore
{{#include ../../exercise-solutions/simple_db/step2/lib.rs}}
```

</details>

### Step 3: Read the documentation for `str`, especially `trim()`, `splitn()`, `split_once()` to build your logic

tl;dr
`split_once()` splits a str into 2 parts at the first occurrence of a delimiter.
`splitn()` splits a str into a max of n substrings at every occurrence of a delimiter.
`trim()` returns a string slice with leading and trailing whitespace removed.

<details>
  <summary>The proposed logic</summary>

Split the input with `split_once()` using `\n` as delimeter, this allows to distiguish 3 cases:

- a command with no `\n` -> Error::IncompleteMessage
- a command with trailing data, where the second substring's length is longer than 0 -> Error::TrailingData
- a command where `\n` is the last part, and the second substring is of length 0 -> generic command

Split the input with `splitn()` using `' '` as delimeter and 2 as the max number of substrings. The method returns `Some(T)` where T is an iterator over the substrings, and `None` when there are no substrings. Note, that even an empty str `""` is a substring. This allows us to distiguish the following cases:

- `Some(T)` contains all methods that have either one or two substrings -> generic Command
- `None` is returned if no substrings are returned -> Error::UnknownError

From here, the actual command cases need to be distiguished with pattern matching:

- `RETRIEVE\n` has no whitespace and no payload
- `PUBLISH <payload>\n` has always whitespace and an optional payload

</details>

### Step 4: Implement `fn parse()`

#### Step 4a: Sorting out wrongly placed and absent newlines

Missing, wrongly placed and more than one `\n` are errors that occur independent of other errors so it makes sense to handle these cases first. Split the incoming message at the first appearing `\n` using `split_once()`. This operation yields `Some((&str, &str))` if at least one `\n` is present, and `None` if 0 are present. If the `\n` is **not** the last item in the message, the second `&str` in `Some((&str, &str))` is longer than 0 bytes.

Tip: Introduce a generic variant `Command::Command` that temporarily stands for a valid command. 

Handle the two cases with match, check the length of the second `&str` with `len()`. Return `Err(Error::TrailingData)` or for wrongly placed `\n`, `Err(Error::IncompleteMessage)` for absent `\n` and `Ok(Command::Command)` if the `\n` is placed correct.

<details>
  <summary>Solution</summary>

```rust, ignore
{{#include ../../exercise-solutions/simple_db/step4a/lib.rs}}
```

</details>

#### Step 4b: `if let`: sorting `Some()` from `None`

In 4a a generic command is distiguished from a message that contains trailing data in an else branch. Remove the else branch before continuing, because we want to distishish this case further. 

Use `.splitn()` to split the `input` into 2 parts at max, use whitespace as delimiter (`' '`). This method yields an iterator over the substrings.

Use `.next()` to access the first substring, the command keyword, which is wrapped into the `Option<T>` type. Sign it with the `Some()` Option to `if let`.

This tests if there is at least one substring in the input.

Return the generic `Ok(Command::Command)` for the `Some()` case, and `Err(Error::UnknownError)` for `None`. The error is unknown, since `None` is only returned if there is nothing to iterate about. Even an empty string would return `Some()`!

<details>
  <summary>Solution</summary>

```rust, ignore
{{#include ../../exercise-solutions/simple_db/step4b/lib.rs}}
```

</details>

#### Step 4c: Pattern matching for the command keywords

Remove the Ok(Command::Command) and the enum variant. Use `.trim()` on the command substring and use `match` to patternmatch its content. `.trim()` removes any `\n` that are in the substring. Next, implement two necessary match arms: `""` for empty messages, `_` for any other string, currently evaluated to be an unknown command.

<details>
  <summary>Solution</summary>

```rust, ignore
{{#include ../../exercise-solutions/simple_db/step4c/lib.rs}}
```

</details>

#### Step 4d: Add Retrieve Case

Add a match arm to check if the command substring is equal to `"RETRIEVE"`. It’s not enough to return `Ok(Command::Retrieve)` just yet. The Retrieve command cannot have a payload, this includes whitespace! To check for this, add an if else statement, that checks if the next iteration over the substrings returns none. If this is true, return the `Ok(Command::Retrieve)`, if it is false, return `Err(Error::UnexpectedPayload)`.

<details>
  <summary>Solution</summary>

```rust, ignore
{{#include ../../exercise-solutions/simple_db/step4d/lib.rs}}
```

</details>

#### Step 4e: Add Publish Case and finish

Add a `match` arm to check if the command substring is equal to `"PUBLISH"`. Just like with the Retrieve command, we need to add a distinction, but the other way round: Publish needs a payload or whitespace for an empty payload to be valid.

Use `if let` to check if the next iteration into the substrings returns `Some()`. If it does, return `Ok(Command::Publish(T))`, where T is an owned version of the trimmed payload. Otherwise return `Err(Error::MissingPayload)`.

<details>
  <summary>Solution</summary>

```rust, ignore
{{#include ../../exercise-solutions/simple_db/step4e/lib.rs}}
```

</details>

## Full source code

If all else fails, feel free to copy this solution to play around with it.

<details>
  <summary>Solution</summary>

```rust
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
```

</details>

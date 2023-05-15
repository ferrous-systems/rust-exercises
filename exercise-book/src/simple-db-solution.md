# Step-by-Step Solution

## Step 1: Creating a library project with cargo

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

## Step 2: Appropriate data structures

Define two enums, one is called `Command` and one is called `Error`. `Command` has 2 variants for the two possible commands. `Publish` carries data (the message), `Retrieve` does not. `Error` is just a list of error *kinds*. Use `#[derive(Eq,PartialEq,Debug)]` for both `enums`.

<details>
  <summary>Solution</summary>

```rust, ignore
{{#include ../../exercise-solutions/simple_db/step2/src/lib.rs}}
```

</details>

## Step 3: Read the documentation for `str`, especially `trim()`, `splitn()`, `split_once()` to build your logic

tl;dr
- `split_once()` splits a str into 2 parts at the first occurrence of a delimiter.
- `splitn()` splits a str into a max of n substrings at every occurrence of a delimiter.
- `trim()` returns a string slice with leading and trailing whitespace removed.

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

## Step 4: Implement `fn parse()`

### Step 4a: Sorting out wrongly placed and absent newlines

Missing, wrongly placed and more than one `\n` are errors that occur independent of other errors so it makes sense to handle these cases first. Split the incoming message at the first appearing `\n` using `split_once()`. This operation yields `Some((&str, &str))` if at least one `\n` is present, and `None` if 0 are present. If the `\n` is **not** the last item in the message, the second `&str` in `Some((&str, &str))` is longer than 0 bytes.

Tip: Introduce a generic variant `Command::Command` that temporarily stands for a valid command. 

Handle the two cases with match, check the length of the second `&str` with `len()`. Return `Err(Error::TrailingData)` or for wrongly placed `\n`, `Err(Error::IncompleteMessage)` for absent `\n` and `Ok(Command::Command)` if the `\n` is placed correct.

<details>
  <summary>Solution</summary>

```rust, ignore
{{#include ../../exercise-solutions/simple_db/step4a/src/lib.rs:19:33}}
```

</details>

### Step 4b: `if let`: sorting `Some()` from `None`

In 4a a generic command is distiguished from a message that contains trailing data in an else branch. Remove the else branch before continuing, because we want to distishish this case further. 

Use `.splitn()` to split the `input` into 2 parts at max, use whitespace as delimiter (`' '`). This method yields an iterator over the substrings.

Use `.next()` to access the first substring, the command keyword, which is wrapped into the `Option<T>` type. Sign it with the `Some()` Option to `if let`.

This tests if there is at least one substring in the input.

Return the generic `Ok(Command::Command)` for the `Some()` case, and `Err(Error::UnknownError)` for `None`. The error is unknown, since `None` is only returned if there is nothing to iterate about. Even an empty string would return `Some()`!

<details>
  <summary>Solution</summary>

```rust, ignore
{{#include ../../exercise-solutions/simple_db/step4b/src/lib.rs:18:38}}
```

</details>

### Step 4c: Pattern matching for the command keywords

Remove the Ok(Command::Command) and the enum variant. Use `.trim()` on the command substring and use `match` to patternmatch its content. `.trim()` removes any `\n` that are in the substring. Next, implement two necessary match arms: `""` for empty messages, `_` for any other string, currently evaluated to be an unknown command.

<details>
  <summary>Solution</summary>

```rust, ignore
{{#include ../../exercise-solutions/simple_db/step4c/src/lib.rs:17:39}}
```

</details>

### Step 4d: Add Retrieve Case

Add a match arm to check if the command substring is equal to `"RETRIEVE"`. It’s not enough to return `Ok(Command::Retrieve)` just yet. The Retrieve command cannot have a payload, this includes whitespace! To check for this, add an if else statement, that checks if the next iteration over the substrings returns none. If this is true, return the `Ok(Command::Retrieve)`, if it is false, return `Err(Error::UnexpectedPayload)`.

<details>
  <summary>Solution</summary>

```rust, ignore
{{#include ../../exercise-solutions/simple_db/step4d/src/lib.rs:17:46}}
```

</details>

### Step 4e: Add Publish Case and finish

Add a `match` arm to check if the command substring is equal to `"PUBLISH"`. Just like with the Retrieve command, we need to add a distinction, but the other way round: Publish needs a payload or whitespace for an empty payload to be valid.

Use `if let` to check if the next iteration into the substrings returns `Some()`. If it does, return `Ok(Command::Publish(T))`, where T is an owned version of the trimmed payload. Otherwise return `Err(Error::MissingPayload)`.

<details>
  <summary>Solution</summary>

```rust, ignore
{{#include ../../exercise-solutions/simple_db/step4e/src/lib.rs:17:53}}
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
```

</details>


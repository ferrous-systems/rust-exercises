# Step-by-Step Solution

## Step 1: Creating a library project with cargo

Create a new Cargo project, check the build and the test setup:

<details>
  <summary>Solution</summary>

```console
cargo new --lib simple-db
cd simple-db
cargo build
cargo test
```

</details>

## Step 2: Appropriate data structures

Define two enums, one is called `Command` and one is called `Error`. `Command` has 2 variants for the two possible commands. `Publish` carries data (the message), `Retrieve` does not. `Error` is just a list of error *kinds*. Use `#[derive(Eq,PartialEq,Debug)]` for both `enums`.

<details>
  <summary>Solution</summary>

```rust, ignore
{{#include ../../exercise-solutions/simple-db/step2/src/lib.rs}}
```

</details>

## Step 3: Read the documentation for `str`, especially `splitn()`, `split_once()` to build your logic

tl;dr
- `split_once()` splits a str into 2 parts at the first occurrence of a delimiter.
- `splitn()` splits a str into a max of n substrings at every occurrence of a delimiter.

<details>
  <summary>The proposed logic</summary>

Split the input with `split_once()` using `\n` as delimiter, this allows to distinguish 3 cases:

- a command where `\n` is the last part, and the second substring is `""` -> some kind of command
- a command with trailing data (i.e. data after a newline) -> Error::TrailingData
- a command with no `\n` -> Error::IncompleteMessage

After that, split the input with `splitn()` using `' '` as delimiter and 2 as the max number of substrings. The method an iterator over the substrings, and the iterator produces `Some(...)`, or `None` when there are no substrings. Note, that even an empty str `""` is a substring.

From here, the actual command cases need to be distinguished with pattern matching:

- `RETRIEVE` has no whitespace and no payload
- `PUBLISH <payload>` has always whitespace and an optional payload

</details>

## Step 4: Implement `fn parse()`

### Step 4a: Sorting out wrongly placed and absent newlines

Missing, wrongly placed and more than one `\n` are errors that occur independent of other errors so it makes sense to handle these cases first. Split the incoming message at the first appearing `\n` using `split_once()`. This operation yields `Some((&str, &str))` if at least one `\n` is present, and `None` if 0 are present. If the `\n` is **not** the last item in the message, the second `&str` in `Some((&str, &str))` is not `""`.

Tip: Introduce a generic variant `Command::Command` that temporarily stands for a valid command.

Handle the two cases with match, check if the second part is `""`. Return `Err(Error::TrailingData)` or for wrongly placed `\n`, `Err(Error::IncompleteMessage)` for absent `\n` and `Ok(Command::Command)` if the `\n` is placed correct.

<details>
  <summary>Solution</summary>

```rust, ignore
{{#include ../../exercise-solutions/simple-db/step4a/src/lib.rs:18:24}}
```

</details>

### Step 4b: `if let`: sorting `Some()` from `None`

In 4a, we produce a `Ok(Command::Command)` if the newlines all check out. Instead of doing that, we want to capture the message - that is the input, without the newline on the end, and we know it has no newlines within it.

Use `.splitn()` to split the `message` into 2 parts maximum, use a space as delimiter (`' '`). This method yields an iterator over the substrings.

Use `.next()` to access the first substring, which is the command keyword. You will always get `Some(value)` - the `splitn` method never returns `None` the first time around. We can unwrap this first value because `splitn` always returns at least one string - but add yourself a comment to remind yourself why this `unwrap()` is never going to fail!

<details>
  <summary>Solution</summary>

```rust, ignore
{{#include ../../exercise-solutions/simple-db/step4b/src/lib.rs:18:30}}
```

</details>

### Step 4c: Pattern matching for the command keywords

Remove the `Ok(Command::Command)` and the enum variant. Use `match` to pattern match the command instead. Next, implement two necessary match arms: `""` for empty messages, `_` for any other string, currently evaluated to be an unknown command.

<details>
  <summary>Solution</summary>

```rust, ignore
{{#include ../../exercise-solutions/simple-db/step4c/src/lib.rs:17:32}}
```

</details>

### Step 4d: Add Retrieve Case

Add a match arm to check if the command substring is equal to `"RETRIEVE"`. It’s not enough to return `Ok(Command::Retrieve)` just yet. The Retrieve command cannot have a payload, this includes whitespace! To check for this, add an if else statement, that checks if the next iteration over the substrings returns `None`. If this is true, return the `Ok(Command::Retrieve)`, if it is false, return `Err(Error::UnexpectedPayload)`.

<details>
  <summary>Solution</summary>

```rust, ignore
{{#include ../../exercise-solutions/simple-db/step4d/src/lib.rs:17:39}}
```

</details>

### Step 4e: Add Publish Case and finish

Add a `match` arm to check if the command substring is equal to `"PUBLISH"`. Just like with the Retrieve command, we need to add a distinction, but the other way round: Publish needs a payload or whitespace for an empty payload to be valid.

Use `if let` to check if the next iteration into the substrings returns `Some()`. If it does, return `Ok(Command::Publish(payload))`, where `payload` is an owned version (a `String`) of the trimmed payload. Otherwise return `Err(Error::MissingPayload)`.

<details>
  <summary>Solution</summary>

```rust, ignore
{{#include ../../exercise-solutions/simple-db/step4e/src/lib.rs:17:46}}
```

</details>

## Full source code

If all else fails, feel free to copy this solution to play around with it.

<details>
  <summary>Solution</summary>

```rust
{{#include ../../exercise-solutions/simple-db/step4e/src/lib.rs}}
```

</details>


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

## Step 2: Define appropriate data structures

Define two enums, one is called `Command` and one is called `Error`. `Command` has 2 variants for the two possible commands. `Publish` carries data (the message), `Retrieve` does not. `Error` is just a list of error *kinds*. Use `#[derive(Eq,PartialEq,Debug)]` for both `enums`.

<details>
  <summary>Solution</summary>

```rust, ignore
{{#include ../../exercise-solutions/simple-db/step2/src/lib.rs}}
```

</details>

## Step 3: Read the documentation for `str`, especially `strip_prefix()`, `strip_suffix()`

tl;dr:

* `message.strip_prefix("FOO ")` returns `Some(remainder)` if the string slice `message` starts with `"FOO "`, otherwise you get `None`
* `message.strip_suffix('\n')` returns `Some(remainder)` if the string slice `message` ends with `'\n'`, otherwise you get `None`.

Note that both functions will take either a string slice, or a character, or will actually even take a function that returns a boolean to tell you whether a character matches or not (we won't use that though).

<details>
  <summary>The proposed logic</summary>

1. Check if the string ends with the char `'\n'` - if so, keep the rest of it, otherwise return an error.

2. Check if the remainder still contains a `'\n'` - if so, return an error.

3. Check if the remainder is empty - if so, return an error.

4. Check if the remainder begins with `"PUBLISH "` - if so, return `Ok(Command::Publish(...))` with the payload upconverted to a `String`

5. Check if the remainder is `"PUBLISH"` - if so, return an error because the mandatory payload is missing.

6. Check if the remainder begins with `"RETRIEVE "` - if so, return an error because that command should not have anything after it.

7. Check if the remainder is `"RETRIEVE"` - if so, return `Ok(Command::Retrieve)`

8. Otherwise, return an unknown command error.

</details>

## Step 4: Implement `fn parse()`

### Step 4a: Sorting out wrongly placed and absent newlines

Missing, wrongly placed and more than one `\n` are errors that occur independent of other errors so it makes sense to handle these cases first. Check the string has a newline at the end with `strip_suffix`. If not, that's an `Error::IncompleteMessage`. We can assume the pattern will match (that `strip_suffix` will return `Some(...)`, which is our so-called *sunny day scenario*) so a *let - else* makes most sense here - although a match will also work.

Now look for newlines within the remainder using the `contains()` method and if you find any, that's an error.

Tip: Introduce a generic variant `Command::Command` that temporarily stands for a valid command.

<details>
  <summary>Solution</summary>

```rust, ignore
{{#include ../../exercise-solutions/simple-db/step4a/src/lib.rs:18:27}}
```

</details>

### Step 4b: Looking for "RETRIEVE"

In 4a, we produce a `Ok(Command::Command)` if the newlines all check out. Now we want to look for a RETRIEVE command.

If the string is empty, that's an error. If the string is exactly `"RETRIEVE"`, that's our command. Otherwise the string *starts with* `"RETRIEVE "`, then that's an *UnexpectedPayload* error.

<details>
  <summary>Solution</summary>

```rust, ignore
{{#include ../../exercise-solutions/simple-db/step4b/src/lib.rs:18:34}}
```

</details>

### Step 4c: Looking for "PUBLISH"

Now we want to see if the message starts with `"PUBLISH "`, and if so, return a `Command::Publish` containing the payload, but converted to a heap-allocated `String` so that ownership is passed back to the caller. If not, and the message is equal to `"PUBLISH"`, then that's a *MissingPayload* error.

<details>
  <summary>Solution</summary>

```rust, ignore
{{#include ../../exercise-solutions/simple-db/step4c/src/lib.rs:18:38}}
```

</details>

## Full source code

If all else fails, feel free to copy this solution to play around with it.

<details>
  <summary>Solution</summary>

```rust
{{#include ../../exercise-solutions/simple-db/step4c/src/lib.rs}}
```

</details>

# Knowledge

This section explains concepts necessary to solve the simpleDB exercise.

In general, we also recommend to use the Rust documentation to figure out things you are missing to familiarize yourself with it. If you ever feel completely stuck or that you haven’t understood something, please hail the trainers quickly.

## Derives

`#[derive(PartialEq, Eq)]`

  This enables comparison between 2 instances of the type, by comparing every field/variant. This enables the `assert_eq!` macro, which relies on equality being defined. `Eq` for total equality isn’t strictly necessary for this example, but it is good practice to derive it if it applies.

`#[derive(Debug)]`

This enables automatic debug output for the type. The `assert_eq!`macro requires this for testing.


## Control flow and pattern matching, returning values

This exercise involves handling a number of cases. You are already familiar with `if/else` and a basic form of `match`. Here, we’ll introduce you to `if let`.

```rust, ignore
    if let Some(payload) = substrings.next() {
        // execute if the above statement is true
    }
```


### When to use what?

`if let` is like a pattern-matching `match` block with only one arm. So, if your `match` only has one arm of interest, consider an `if let` instead.

`match` can be used to handle more fine grained and complex pattern matching, especially when there are several, equally ranked possibilities. The match arms may have to include a catch all `_ =>` arm, for every possible case that is not explicitly spelled out. The order of the match arms matter: The catch all branch needs to be last, otherwise, it catches all…

### Returning Values from branches and match arms

All match arms always need to produce a value the same type (or they diverge with a `return` statement).

<!-- markdownlint-disable MD033 -->
# Calculator

In this exercise we will implement a Reverse-Polish-Notation Calculator library.

You will learn:

* How to write libraries in Rust
* How to use tests to check library code
* How to model tree-like data structures in Rust using `enum`
* How to use `enum` types to model custom errors
* How to use turbofish syntax to assist type resolution
* How to handle simple text parsing in Rust

## Task

Write a library that parses and evaluates math expressions that use [Reverse-Polish (postfix) notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation).
For example, a string `"3 1 + 2 /"` in this notation is equivalent to `(3 + 1) / 2` but unlike the later it does not force us to handle operator precedence using parentheses.

We will support the 4 basic math operations (`+`, `-`, `*`, `/`) that all will expect two operands, and a `sqr` operation to square a single number.

Here's a basic grammar of expressions that we can expect:

```rust ignore
expr =
    number
    | expr expr '+'
    | expr expr '-'
    | expr expr '*'
    | expr expr '/'
    | expr 'sqr'
```

and here are some examples for you:

| Postfix Notation | Traditional notation | Value |
|------------------|----------------------|-------|
| 42               | 42                   | 42    |
| 40 2 +           | 40 + 2               | 42    |
| 1 3 + 2 /        | (1 + 3) / 2          | 2     |

**Hint:** you can use these examples as basis for unit tests.

Our library should expose a pair of functions:

```rust ignore
// Takes a string and produces an expression tree
pub fn parse(input: &str) -> Result<Expr, ParseError>

// Evaluates the expression
pub fn eval(expr: &Expr) -> Result<i64, ParseError>
```

### `Expr` type

The `Expr` type has to represent any math expression we can encounter, from a simple number to a complex nested expressions.
We will use an `enum` type for different expression variants.

Note how we use `Expr` inside a `Square` variant.
This way we can represent both simple expressions (like `5 sqr`) and something very complex (like `5 sqr sqr sqr` or `3 2 + sqr`).

```rust ignore
enum Expr {
    Number(i64),
    Square(Box<Expr>),
    // ...
}

// Here's how a `Square` variant can be created:
let four = Expr::Square(Box::new(Expr::Number(2)));
```

### Here's a bit of code to get you started

```rust ignore
pub enum Expr {}

pub enum ParseError {}

pub fn parse(input: &str) -> Result<Expr, ParseError> {
    let mut stack: Vec<Expr> = Vec::new();
    for word in input.split_ascii_whitespace() {

    };
    assert_eq!(stack.len(), 1);
    let res = stack.pop().unwrap();
    Ok(res)
}

pub enum EvalError {}

pub fn eval(expr: &Expr) -> Result<i64, EvalError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        let input = "42";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 42);
    }

    // #[test]
    // fn smoke_test() {
    //     let input = "3 sqr 4 sqr + 5 sqr -";
    //     let expr = parse(input).unwrap();
    //     let value = eval(&expr).unwrap();
    //     assert_eq!(value, 0);
    // }
}
```

**Tip: When working on this exercise it's often easier to add support for one operation at a time.**

1. Write a new test for an operation
2. Add support for parsing the new kind of expression
3. Add support for this kind of operation to `eval` function

This way your code will be in a working state regularly, and iterating on it will be easier.

**Tip: the starting code is not untouchable!**

You can rewrite bits of it as you see fit.
For example, the assertion in the `parse` function can go in a way of your tests.
Feel free to comment it out, or better yet refactor the bit at the end to get rid of it and the `unwrap` call.

### Stretch goals

* Handle overflow and underflow errors in `eval` function ([`checked_add`](https://doc.rust-lang.org/std/primitive.i64.html#method.checked_add) and similar methods can be very useful here).
* Add support for unary minus `-` operator.
* *Hard:* change `parse` function to support infix notation, operator precedence, and parentheses.

## Help

### Recursive data structures in Rust

<p>
<details>
<summary>
What does <code>Box</code> mean in the <code>Expr</code> type?
</summary>

If you try to make a `enum` type that uses itself as a field in one of its variants, then the type will potentially have an infinite size.
And if you would try to make a local variable of that type, the compiler wouldn't know how big [the stack frame](https://en.wikipedia.org/wiki/Call_stack#Structure) for that function would have to be.
To avoid this we introduce an indirection via a [`Box` wrapper](https://doc.rust-lang.org/rust-by-example/std/box.html).
It forces the wrapped portion of the type to be heap-allocated, and the size of `Box` itself becomes predictable.

[`Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html) is not the only type you can use for this purpose.
[`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html) and [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) are other examples of these [*smart pointer* types](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html).
And in addition, `Vec` from standard library holds all its items on a heap, too.
</details>
</p>

### Parsing postfix notation text

<p>
<details>
<summary>
What's the idea behind <code>stack</code> variable?
</summary>

We offer a rough scaffold for the `parse` function in the starter code.
As we split the input around spaces we will get either a number or an operator at a time.
We will use `stack` variable to store temporary sub-expressions.

When we encounter an operator we pop one or two expressions from the stack, wrap them into a new expression, and then push the it back to the `stack`.
When we encounter a number we simply wrap it into `Expr::Number(...)` and push it.

At the end, if the original string was well-formed we should end up with just a single item in the `stack` representing the whole expression.

To parse numbers you can use the [`parse()` method on string slices](https://doc.rust-lang.org/std/primitive.str.html#method.parse).
This method can produce values of many different types, and you can use *turbofish* syntax to give the compiler a hint about what kind of value you expect:

```rust ignore
let value = "12".parse::<i64>()?;
```

</details>
</p>

### Dealing with `Option` and `Result` types

<p>
<details>
<summary>
Should you <code>unwrap()</code>?
</summary>

In the `parse` function there are many instances where you either get an `Option` or an `Result` with an error that is *different* from `ParseError`.
While calling `unwrap()` in these cases can be tempting there are better options.

In general, unless there's some way you can recover from an error in your function *you should prefer `?` operator* to bail out of it when things go wrong.
Thus, when getting an `Option` or a `Result` your first though should be: "I want to use `?`. How do I get there?"

Thankfully, `Result` has a convenient `map_err` method that can convert the error you get into an error type that you need:

```rust ignore
file.read_to_string(&mut buffer).map_err(|io_error| MyError::IoVariant)?;
```

Likewise, `Option` has a helpful `ok_or` method to convert to `Result` type that you can then use `?` on.

```rust ignore
100_u8.checked_add(200_u8).ok_or(MyError::ByteOverflow)?;
```

`Option` and `Result` have many other useful methods, and given that these types show up in Rust code all the time learning their API will help you writing more terse and idiomatic code.
You can read more about them in [*Item 3 of Effective Rust* book](https://effective-rust.com/transform.html).

In addition, you can always `match` / `let else` / `if let` on your `Option`s and `Result`s in tricky situations.
</details>
</p>

### Testing error cases

<p>
<details>
<summary>
Use <code>unwrap_err()</code> method
</summary>

While the use of panicking functions like `unwrap` in production code is often frowned upon they are popular for tests.
Similar to how you can use `unwrap` to get a value out of `Ok` variant of a `Result` you can use `unwrap_err` to get a value out of `Err`.
Here's an example:

```rust ignore
#[test]
fn no_a_number() {
    let input = "X";
    let error = parse(input).unwrap_err();
    assert_eq!(error, ParseError::NotANumber);
}
```

</details>
</p>

### Make your types testable

<p>
<details>
<summary>
Use <code>derive</code> for your types
</summary>

Rust's testing macro`assert_eq!` compares the two arguments using `==` operator and if the values do not match it prints them in `Debug` mode to show you the difference between them.
Thus, to use it with your types like `Expr` the types have to be comparable and printable, i.e. implement [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) and [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) traits.
Thankfully, both traits can usually be derived automatically like so:

```rust ignore
#[derive(Debug, PartialEq)]
pub enum Expr {
    // ...
}
```

</details>
</p>

### Type conversions

<p>
<details>
<summary>
<code>&i64</code> to <code>i64</code>
</summary>

The `eval` function takes `&Expr` type as an argument and when you use it in `match` you will get `&i64` instead of `i64`.
You can use a dereferencing operator `*` to convert *a reference to a number* to *a number itself*:

```rust ignore
// expr: &Expr
match expr {
    // n: &i64
    Expr::Number(n) => {
        let x: i64 = *n;
    }
}
```

</details>
<p>

<p>
<details>
<summary>
<code>&Box&ltExpr></code> to <code>&Expr</code>
</summary>

Rust will make this conversion automatically, you don't have to do anything!
To learn more about how it works read about [`Deref` coercion in The Rust Book](https://doc.rust-lang.org/book/ch15-02-deref.html).

</details>
<p>

<p>
<details>
<summary>
<code>Box&ltExpr></code> to <code>Expr</code>
</summary>

Similar to `&i64` to `i64` you can use a dereferencing operator:

```rust ignore
let boxed_expression: Box<Expr> = Box::new(Expr::Number(1));
let expr: Expr = *boxed_expression;
```

</details>
<p>

## Step by step solution

<details>
<summary>
Click to see the steps.

[A full solution is available in our repository.](../../exercise-solutions/calculator/)
</summary>

### Step 1: Make a new library

```bash
cargo new --lib calc
```

Paste the starting code into `lib.rs`.
Add `#[derive(Debug, PartialEq)]` for `Expr` and error types to fix compilation errors in the test.

### Step 2: Add support for numbers

Let's start with `parse` function.
Inside a `for` loop we'll be matching `word` variable against different operators.
If none of the operators match we will assume that we encounter a number.
So, number parsing will be in the default branch of our match expression:

```rust ignore
// inside `parse`
for word in input.split_ascii_whitespace() {
    match word {
        "+" => todo!("add support for different operators"),
        _ => {
            let number = word
                .parse::<i64>()
                .map_err(|_| ParseError::NotANumber(word.to_string()))?;
            let expr = Expr::Number(number);
            stack.push(expr);
        }
    }
}
```

**Tip:** You can use Rust Analyzer to populate enum types.
When you type `ParseError::NotANumber(word.to_string())` put the cursor over `NotANumber` and use *Generate Variant* quick action.
You can do it again after typing `Expr::Number`.

Now, let's work on `eval` function.
After you type `match expr` in the body you once again can use a quick action to generate a missing match arm for `Number` variant.

```rust ignore
// in `eval`
match expr {
    Expr::Number(n) => Ok(*n),
}
```

You can now run the number test that we provide.

You can also write an error text for `ParseError::NotANumber`:

```rust ignore
// in `tests` module:
#[test]
fn not_a_number() {
    let input = "X";
    let error = parse(input).unwrap_err();
    assert_eq!(error, ParseError::NotANumber("X".to_string()));
}
```

### Step 3: Add support for addition

You can start with any operator of your choosing.
Some of them may be trickier than others:

* `sqr` takes only one argument
* `-` and `/` take the order of the operands in account
* `/` can produce an error during evaluation

So, depending on what operator we will implement first our work can be easier (`sqr`) or harder (`/`).
Addition seems like a good starting place.

`parse`:

```rust ignore
match word {
    "+" => {
        let a = stack.pop().ok_or(ParseError::MissingOperand)?;
        let b = stack.pop().ok_or(ParseError::MissingOperand)?;
        let a = Box::new(a);
        let b = Box::new(b);
        let expr = Expr::Add(a, b);
        stack.push(expr);
    }
    _ => {
        // number parsing code
    }
}
```

Once again: use *Generate Variant* for the new kind of expression.

`eval`:

```rust ignore
match expr {
    Expr::Number(n) => Ok(*n),
    Expr::Add(a, b) => {
        let a = eval(a)?;
        let b = eval(b)?;
        Ok(a + b)
    }
}
```

and tests:

```rust ignore
#[test]
fn add() {
    let input = "40 2 +";
    let expr = parse(input).unwrap();
    let value = eval(&expr).unwrap();
    assert_eq!(value, 42);
}
```

### Step 4: Add support for squaring a number

You can probably notice that both `parse` and `eval` functions open opportunities for a refactoring.
Before doing that we should probably explore the variant of expression with *the largest potential* of being different.
`sqr` operator will produce code of *a different shape* due to it requiring only a single operand.
In real world situations it is sometimes hard to predict how different requirements will shape the resulting code.
However, making *an attempt* at a prediction like this can help you with avoiding refactoring prematurely and having to rollback massive changes later.

`parse`:

```rust ignore
match word {
    "+" => {
        // ...
    }
    "sqr" => {
        let a = stack.pop().ok_or(ParseError::MissingOperand)?;
        let a = Box::new(a);
        let expr = Expr::Square(a);
        stack.push(expr);
    }
    _ => {
        // ...
    }
}
```

`eval`:

```rust ignore
match expr {
    // ...
    Expr::Square(a) => {
        let a = eval(a)?;
        Ok(a.pow(2))
    }
}
```

Test:

```rust ignore
#[test]
fn square() {
    let input = "5 sqr";
    let expr = parse(input).unwrap();
    let value = eval(&expr).unwrap();
    assert_eq!(value, 25);
}
```

### Step 5: Refactoring

#### `parse` function

Some observations:

* Every branch of our big `match` statement ends up producing an expression.
* Every time we `pop` expressions from the stack we have to box them.

So, here's a plan:

* Let's make `match` produce an expression: `let expr = match word { ... };`
* Let's change our `stack` variable to store `Box`ed expressions

```rust ignore
pub fn parse(input: &str) -> Result<Expr, ParseError> {
    let mut stack: Vec<Box<Expr>> = Vec::new();
    for word in input.split_ascii_whitespace() {
        let expr = match word {
            "+" => {
                let a = stack.pop().ok_or(ParseError::MissingOperand)?;
                let b = stack.pop().ok_or(ParseError::MissingOperand)?;
                Expr::Add(a, b)
            }
            "sqr" => {
                let a = stack.pop().ok_or(ParseError::MissingOperand)?;
                Expr::Square(a)
            }
            _ => {
                let number = word
                    .parse::<i64>()
                    .map_err(|_| ParseError::NotANumber(word.to_string()))?;
                Expr::Number(number)
            }
        };
        stack.push(Box::new(expr));
    }
    assert_eq!(stack.len(), 1);
    let res = stack.pop().unwrap();
    Ok(*res)
}
```

Note that we have to adjust the type from `Box<Expr>` to `Expr` at the end.
While we are at it let's rewrite the end portion of the function to get rid of an assertion and an `unwrap`:

```rust ignore
for word in input.split_ascii_whitespace() {
    // ...
}

match stack.pop() {
    Some(expr) if stack.is_empty() => Ok(*expr),
    Some(_) => Err(ParseError::TooManyOperands),
    None => Err(ParseError::EmptyInput),
}
```

Let's test our new error conditions:

```rust ignore
#[test]
fn too_many_operands() {
    let input = "42 42 42 +";
    let error = parse(input).unwrap_err();
    assert_eq!(error, ParseError::TooManyOperands);
}

#[test]
fn empty_input() {
    let input = "      ";
    let error = parse(input).unwrap_err();
    assert_eq!(error, ParseError::EmptyInput);
}
```

#### `eval` function

So far every branch of the `match` produces an number that we later wrap into `Ok`.

```rust ignore
pub fn eval(expr: &Expr) -> Result<i64, EvalError> {
    let value = match expr {
        Expr::Number(n) => *n,
        Expr::Add(a, b) => eval(a)? + eval(b)?,
        Expr::Square(a) => eval(a)?.pow(2),
    };
    Ok(value)
}
```

### Step 6: Subtraction

In `parse` function all subsequent operators will require two `stack.pop()` calls.
We may as well combine different operators together and use `unreachable!` macro for the second match:

```rust ignore
match word {
    "+" | "-" => {
        let a = stack.pop().ok_or(ParseError::MissingOperand)?;
        let b = stack.pop().ok_or(ParseError::MissingOperand)?;
        match word {
            "+" => Expr::Add(a, b),
            "-" => Expr::Sub(a, b),
            _ => unreachable!(),
        }
    }
    // ...
}
```

```rust ignore
// in `eval`
match expr {
    // ...
    Expr::Add(a, b) => eval(a)? + eval(b)?,
    Expr::Sub(a, b) => eval(a)? - eval(b)?,
    // ...
};
```

```rust ignore
#[test]
fn sub() {
    let input = "42 2 -";
    let expr = parse(input).unwrap();
    let value = eval(&expr).unwrap();
    assert_eq!(value, 40);
}
```

Adding a test will reveal that we have a bug with the order of operands.

```text
---- tests::sub stdout ----
thread 'tests::sub' panicked at calculator/src/lib.rs:96:9:
assertion `left == right` failed
  left: -40
  right: 40
```

It's up to you to decide where to mitigate the issue.
You can do it in `eval` or in `parse`.
We will do it in `parse` right away by popping `b` from the stack first:

```rust ignore
match word {
    "+" | "-" => {
        let b = stack.pop().ok_or(ParseError::MissingOperand)?;
        let a = stack.pop().ok_or(ParseError::MissingOperand)?;
        // ...
    }
    // ...
}
```

### Step 7: Multiplication

The changes will completely match code for addition and subtraction.

Here's a test for you to check your work:

```rust ignore
#[test]
fn mul() {
    let input = "21 2 *";
    let expr = parse(input).unwrap();
    let value = eval(&expr).unwrap();
    assert_eq!(value, 42);
}
```

### Step 8: Division

Division will require more code in the `eval` function to check if the divisor is Zero.
You can perform the check inside the `Expr::Div` arm or move it to its own arm and use a guard expression like this:

```rust ignore
match expr {
    // ...
    Expr::Div(_, divisor) if eval(&divisor)? == 0 => {
        return Err(EvalError::DivisionByZero)
    }
    Expr::Div(a, b) => eval(a)? / eval(b)?,
    // ...
}
```

Let's test our code:

```rust ignore
#[test]
fn div() {
    let input = "84 2 /";
    let expr = parse(input).unwrap();
    let value = eval(&expr).unwrap();
    assert_eq!(value, 42);
}

#[test]
fn divide_by_zero() {
    let input = "42 0 /";
    let expr = parse(input).unwrap();
    let error = eval(&expr).unwrap_err();
    assert_eq!(error, EvalError::DivisionByZero);
}
```

Finally, you can uncomment the smoke test and see how our library works for complex expressions.

[You can find a complete solution in our repository.](../../exercise-solutions/calculator/)

</details>

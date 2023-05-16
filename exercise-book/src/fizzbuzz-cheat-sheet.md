# Fizzbuzz Cheat Sheet

This is a syntax cheat sheet to be used with the Fizzbuzz exercise.

## Variables

```rust
let thing = 42; // an immutable variable
let mut thing = 43; // a mutable variable
```

## Functions

```rust
// a function with one argument, no return.
fn number_crunch(input: u32) {
    // function body
}

// a function with two arguments and a return type.
fn division_machine(dividend: f32, divisor: f32) -> f32 {
    // function body
    let quotient = dividend / divisor;

    // return line does not have a semi-colon!
    quotient
}

fn main() {
    
    let cookies = 1000.0_f32;
    let cookie_monsters = 1.0_f32;

    // calling a function 
    let number = division_machine(cookies, cookie_monsters);
}
```

## `for` loops and ranges

```rust
// for loop with end-exclusive range
for i in 0..10 {
    // do this
}

// for loop with end-inclusive range
for j in 0..=10 {
    // do that 
}
```

## if - statements

```rust
let number = 4;

if number == 4 {
    println!("This happens");
} else if number == 5 {
    println!("Something else happens");
} else {
    println!("Or this happens");
}

// condition can be anything that evaluates to a bool

```

## Operators (Selection)

|Operator       |Example            |Explanation                    |
|---------      |---------          |---------                      |
|`!=`           |`expr != expr`     |Nonequality comparison         |
|`==`           |`expr == expr`     |Equality comparison            |
|`&&`           |`expr && expr`     |Short-circuiting logical AND   |
|`\|\|`         |`expr \|\| expr`   |Short-circuiting logical OR    |
|`%`            |`expr % expr`      |Arithmetic remainder           |
|`/`            | `expr / expr`     |Arithmetic division            |

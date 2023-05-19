# Shapes

In this exercise we're going to define methods for a struct, define and implement a trait, and look into how to make these generic. 


You will learn:

## Learning Goals

You will learn how to:

* implement methods for a `struct`
* when to use `Self`, `self`, `&self` and `&mut self` in methods
* define a trait with required methods
* make a type generic over `T`
* how to constrain `T`

## Tasks

### Part 1: Defining Methods for Types

You can find a [complete solution](../../exercise-solutions/shapes-part-1/)

1. Make a new library project called `shapes`
2. Make two structs, `Circle` with field `radius` and `Square` with field `side` to use as types. 
3. Make an `impl` block and implement the following methods for each type. Consider when to use `self`, `&self`, `&mut self` and `Self`.

    * `fn new(...) -> ...`
        * creates an instance of the shape with a certain size (radius or side length).

    * `fn area(...) -> ...`
        * calculates the area of the shape.

    * `fn scale(...)`
        * changes the size of an instance of the shape.

    * `fn destroy(...) -> ...`
        * destroys the instance of a shape and returns the value of its field.

### Part 2: Defining and Implementing a Trait

You can find a [complete solution](../../exercise-solutions/shapes-part-2/)

1. Define a Trait `HasArea` with a mandatory method:  `fn area()`. Implement `HasArea` for `Square`
2. Abstract over `Circle` and `Square` by defining an enum `Shapes` that contains both as variants.

### Part 3: Making `Square` generic over T

You can find a [complete solution](../../exercise-solutions/shapes-part-3/)

We want to make `Square` generic over `T`, so we can use other numeric types and not just `u32`.

1. Add the generic type parameter `<T>` to `Square`.
2. Import the `num` crate, version 0.4.0, in order to be able to use the `Num` trait as bound for the generic type `<T>`. This assures, whatever type is used for `T` is a numeric type and also makes some guarantees about operations that can be performed.
3. Add the restraint to a `where` clause:

```rust, ignore
where
    T: num::Num 
```

4. Depending on the math operation, you may need to add further trait bounds, such as `Copy` and `std::ops::MulAssign`. You can add them to the `where` clause with a `+` sign.

Implement the trait `HasArea` for `Circle`.

Todo! There is a problem with multiplying f32s and gaining T from that. 

## Help

This section gives partial solutions to look at or refer to.

In general, we also recommend to use the Rust documentation to figure
out things you are missing to familiarize yourself with it. If you ever
feel completely stuck or that you haven’t understood something, please
hail the trainers quickly.

### Getting Started

Create a new library Cargo project, check the build and see if it runs:

```
cargo new --lib shapes 
cd shapes \
cargo run
```

### Creating a Type

Each of your shape types (Square, Circle, etc) will need some fields (or
properties) to identify its geometry. Use `///` to add documentation to
each field.

```rust, ignore
    /// Describes a human individual
    struct Person {
        /// How old this person is
        age: u8
    }
```

### Functions that take arguments: self, &self, &mut self

Does your function need to take ownership of the shape in order to calculate its area? Or is it sufficient to merely take a read-only look at the shape for a short period of time?

You can pass arguments **by reference** in Rust by making your function take `x: &MyShape`, and passing them with `&my_shape`.

You can also associate your function with a specific type by placing it inside a block like `impl MyShape { ... }`

```rust, ignore
    impl Pentagon {
        fn area(&self) -> u32 {
            // calculate the area of the pentagon here...
        }
    }
```

### A Shape of many geometries

You can use an `enum` to provide a single type that can be any of your supported shapes. If we were working with fruit, we might say:

```rust
    struct Banana { ... }
    struct Apple { ... }

    enum Fruit {
        Banana(Banana),
        Apple(Apple),
    }
```

### I need a Pi

The `f32` type also has its own module in the standard library called `std::f32`. If you look at the docs, you will find a defined constant for Pi: `std::f32::consts::PI`

### Defining a `Trait`

A trait has a name, and lists function definitions that make guarantees about the name of a method, it's arguments and return types. 

```rust, ignore
pub trait Color {
    fn red() -> u8;
}
```

### Adding generic Type parameters

```rust, ignore
pub struct Square<T> {
    /// The length of one side of the square
    side: T,
}

impl<T> Square<T> {
    // ...
}
```

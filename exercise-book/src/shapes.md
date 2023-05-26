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
2. Make two structs, `Circle` with field `radius` and `Square` with field `side` to use as types. Decide on appropriate types for `radius` and `width`.
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

1. Define a Trait `HasArea` with a mandatory method: `fn area(&self) -> f32`.
2. Implement `HasArea` for `Square` and `Circle`. You can defer to the existing method but may need to cast the return type.
3. Abstract over `Circle` and `Square` by defining an enum `Shape` that contains both as variants.
4. Implement `HasArea` for `Shape`.

### Part 3: Making `Square` generic over `T`

You can find a [complete solution](../../exercise-solutions/shapes-part-3/)

We want to make `Square` and `Circle` generic over `T`, so we can use other numeric types and not just `u32` and `f32`.

1. Add the generic type parameter `<T>` to `Square`. You can temporarily remove `enum Shape` to make this easier.
2. Import the `num` crate, version 0.4.0, in order to be able to use the `num::Num` trait as bound for the generic type `<T>`. This assures, whatever type is used for `T` is a numeric type and also makes some guarantees about operations that can be performed.
3. Add a `where` clause on the methods of `Square`, as required, e.g.:

   ```rust, ignore
   where T: num::Num 
   ```

4. Depending on the operations performed in that function, you may need to add further trait bounds, such as `Copy` and `std::ops::MulAssign`. You can add them to the `where` clause with a `+` sign, like `T: num::Num + Copy`.
5. Add the generic type parameter `<T>` to `Circle` and then appropriate `where` clauses.
6. Re-introduce `Shape` but with the generic type parameter `<T>`, and then add appropriate `where` clauses.

## Help

This section gives partial solutions to look at or refer to.

In general, we also recommend to use the Rust documentation to figure
out things you are missing to familiarize yourself with it. If you ever
feel completely stuck or that you haven’t understood something, please
hail the trainers quickly.

### Getting Started

Create a new library Cargo project, check the build and see if it runs:

```console
$ cargo new --lib shapes 
$ cd shapes
$ cargo run
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

```rust ignore
struct Banana { ... }
struct Apple { ... }

enum Fruit {
    Banana(Banana),
    Apple(Apple),
}
```

If you wanted to count the pips in a piece of Fruit, you might just call to the `num_pips()` method on the appropriate constituent fruit. This might look like:

```rust ignore
impl Fruit {
    fn num_pips(&self) -> u8 {
        match self {
            Fruit::Apple(apple) => apple.num_pips(),
            Fruit::Banana(banana) => banana.num_pips(),
        }
    }
}
```

### I need a π

The `f32` type also has its own module in the standard library called `std::f32`. If you look at the docs, you will find a defined constant for π: `std::f32::consts::PI`.

### I need a π, of type `T`

If you want to convert a Pi constant to some type T, you need a `where` bound like:

```rust ignore
where T: num::Num + From<f32>
```

This restricts T to values that can be converted *from* an `f32` (or, types you can convert an `f32` *into*). You can then call `let my_pi: T = my_f32_pi.into();` to convert your `f32` value into a `T` value.

### Defining a `Trait`

A trait has a name, and lists function definitions that make guarantees about the name of a method, it's arguments and return types. 

```rust
pub trait Color {
    fn red() -> u8;
}
```

### Adding generic Type parameters

```rust
pub struct Square<T> {
    /// The length of one side of the square
    side: T,
}

impl<T> Square<T> {
    // ...
}
```

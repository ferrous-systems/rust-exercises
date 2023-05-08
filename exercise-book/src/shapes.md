# Shapes
In this exercise,  **Shapes**.
Our **Shapes** library lets us work with basic geometric shapes in our
Rust applications.

You will learn:

## Learning Goals

* implement methods for a `struct`
* when to use Self, self, &self and &mut self in methods
* define a Trait with required methods
* make a type generic over T
* how to constrain T

## Tasks

### Part 1
    
1. Open the template that has two shapes defined as structs, `Circle` and `Square`. 
2. Implement the following methods for each type:

    -   `fn new(...) -> Self

    -   `fn area(&self) -> ...`

    -   `fn perimeter(&self) -> ...`
3. 
4. 
* 
* leave out the `enum Shapes`, come up with a `Trait Shape`  with required methods instead. (does this make sense regarding part 2)
* focus on when when to use Self, self, &self and &mut self.
    * add a scale method because it changes the shape
    * add a method that consumes the shape (what makes sense here? a "delete" method?)

### Part 2
* change the square so it's generic over T 
* use word (eg. Unit) instead of <T>  as Type parameter?





        

1.  Write some unit tests

2.  Write some `enum Shape { ... }` which abstracts over those shapes.

3.  Implement the `area` and `perimeter` functions for your `Shape`
    type.

Help
----

This section gives partial solutions to look at or refer to.

In general, we also recommend to use the Rust documentation to figure
out things you are missing to familiarise yourself with it. If you ever
feel completely stuck or that you haven’t understood something, please
hail the trainers quickly.

Getting Started
---------------

Create a new library Cargo project, check the build and see if it runs:

$ cargo new --lib shapes $ cd shapes $ cargo run&lt;/programlisting&gt;

Creating a Type
---------------

Each of your shape types (Square, Circle, etc) will need some fields (or
properties) to identify its geometry. Use `///` to add documentation to
each field.

    /// Describes a human individual
    struct Person {
        /// How old this person is
        age: u8
    }

Functions that take arguments
-----------------------------

Does your function need to take ownership of the shape in order to
calculate its area? Or is it sufficient to merely take a read-only look
at the shape for a short period of time?

You can pass arguments **by reference** in Rust by making your function
take `x: &MyShape`, and passing them with `&my_shape`.

You can also associate your function with a specific type by placing it
inside a block like `impl MyShape { ... }`

    impl Pentagon {
        fn area(self: &Pentagon) -> u32 {
            // calculate the area of the pentagon here...
        }
    }

A Shape of many geometries
--------------------------

You can use an `enum` to provide a single type that can be any of your
supported shapes. If we were working with fruit, we might say:

    struct Banana { ... }
    struct Apple { ... }

    enum Fruit {
        Banana(Banana),
        Apple(Apple),
    }

Which shape do I have?
----------------------

A `match` expression will let you determine which **variant** your
**enum** currently has. Again, using fruit as an example:

    enum Fruit {
        Banana(Banana),
        Apple(Apple)
    }

    impl Fruit {
        fn some_function(self: &Fruit) {
            match self {
                Fruit::Banana(banana) => { ... }
                Fruit::Apple(apple) => { ... }
            }
        }
    }

Remember, a match expression is all about **pattern matching**, not
testing for equality.

I need a Pi, and a Square Root
------------------------------

The `f32` type also has its own module in the standard library called
`std::f32`. If you look at the docs, you will find a defined constant
for Pi, and some useful functions for performing mathematical functions
like square-root.

    let x: f32 = 25.0;
    let y = x.sqrt();
    let z = x * x * std::f32::consts::PI;

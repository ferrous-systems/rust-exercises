# Cpp Interop Exercise

## After completing this exercise you will be able to

- use `autocxx` to deal with C++ data types from Rust in a header-only library
- write your own thin wrappers for C++ functions with default parameters
- be able to call template C++ functions from Rust with `autocxx`


## Prerequisites

- Advanced Rust
- FFI
- C++

## Knowledge

### Macros and build.rs

### Unique Ptr and RAII

### The bindgen wrapper.h trick

### Dealing with C++ default Parameters

## Tasks

Use `autocxx` to develop bindings to a `rapidcsv.h` and print out the dates in `example.csv`, from within Rust as well as the `RowCount`.

You should get:

```console
5
2017-02-24
2017-02-23
2017-02-22
2017-02-21
2017-02-17
```

### Setup

* Add `autocxx`, `build.rs`, `example.csv`, `rapidcsv.h` or look at our template

### Tool Failure

* `Document::new` is not generated
* Perhaps we can... (write out structs)

### Write our wrapper.h

* Write a `open_csv` that will act as our constructor in `wrapper.h`
* Call `doc.GetRowCount();`

### Write GetStringCell














# Cpp Interop Exercise

## After completing this exercise you will be able to

- use `autocxx` to deal with C++ data types from Rust in a header-only library
- write your own thin wrappers for C++ functions with default parameters
- be able to call template C++ functions from Rust with `autocxx`


## Prerequisites

- FFI
- Some knowledge of C/C++

This exercise does not require writing vast amounts of Rust code, but does require understanding how different parts of the build system in Rust and C++ interact to get the project going.

## Tasks

1. Use `autocxx` to develop bindings to a `rapidcsv.h` and print out the dates in `example.csv`, from within Rust as well as the `RowCount`.

You should get:

```console
6
2017-02-24
2017-02-23
2017-02-22
2017-02-21
2017-02-17
```

Because there are 5 rows of data and a header row.

2. Use `autocxx` to read `weather.csv` and report average temperature in June:

```console
1.1999999999999995
```

A full solution is available at `rust-exercises/exercises-solutions/cpp-interop`.

## Knowledge

We will learn to use `autocxx` to interface Rust and C++, a usually onerous affair.

Modern C++ dependencies can have exceptions, their own heap, variadic templates, generic parameters and all sorts of functionality that is hard to interface with.

`autocxx` helps us automate this bridge, up to a point, and we'll see how to overcome its limitations and understand where they come from.

To get started, we'll download a header-only C++ library called [rapidcsv](https://github.com/d99kris/rapidcsv) inside `rust-exercises/exercise-templates/cpp-interop` that can load and process CSV files.

`autocxx` works by inspecting the declared data structures  in `rapidcsv.h` and generating the necessary methods and types from the Rust side.

The types and methods for `rapidcsv::Document`, the main data type that holds a CSV, can be generated with

```rust [], ignore
autocxx::include_cpp! {
    #include "wrapper.h"
    generate!("rapidcsv::Document")
    safety!(unsafe_ffi)
}
```

### build.rs

In order to setup `autocxx`, one needs a `src/build.rs` script that will regenerate the necessary bindings on each `cargo build` invocation if `main.rs` changed.

We've taken a very basic script from the `autocxx` repo and dropped it into `src/build.rs`.

`build.rs` files are scripts that are run before the rest of your program, and we'll use it to link with 3rd party code dependencies.

### CxxStrings

We'll define `CxxStrings` with the use of the handy macro

```rust [], ignore
let_cxx_string!(file_name = "example.csv");
```

which comes with `cxx`. This can become a very painful constructor otherwise since we'd have to turn our Rust strings into `CxxStrings` for opening a file with a C++ method, so this is very handy.

Note that converting these to `&str`s uses at most a fat pointer as overhead, but incurs no further memory allocation on the string object itself.

### Unique Ptr and RAII

Interfacing with C++ normally means that we have to keep track of which data types are allocated on the Rust or C++ heap if we were writing our bindings manually.

`autocxx` is smart enough to help us get RAII for our objects (defined by C++ code) managed by Rusty wrapper types, like `UniquePtr<CxxString>`.

This means that we can have a Rusty-guarantee that the objects we create on the Rust side are cleaned up after we're done.

### The bindgen wrapper.h trick, generics, and default parameters

In order to facilitate `autocxx`'s discovery of the required types we want, we need to provide a single entry point by using the "famous" wrapper.h trick described in the [bindgen docs](https://rust-lang.github.io/rust-bindgen/tutorial-2.html).

This is because `autocxx` (nor any tool, probably) can't concretize generic parameters in the general case, so we need to specify a concrete type signature we will be using:

```c++
#include "rapidcsv.h"

namespace my_csv {
    rapidcsv::Document open_csv(const std::string& pPath) {
        return rapidcsv::Document(pPath);
    }
}
```

Note that we even have access to namespaces!

This trick is also useful for dealing with the `rapidcsv::Document` C++ constructor that has many default parameters:

```c++
explicit Document(const std::string& pPath = std::string(),
                  const LabelParams& pLabelParams = LabelParams(),
                  const SeparatorParams& pSeparatorParams = SeparatorParams(),
                  const ConverterParams& pConverterParams = ConverterParams(),
                  const LineReaderParams& pLineReaderParams = LineReaderParams())
```

`autocxx` does not pick up on the `rapidcsv::Document` constructor having all these default parameters.

If we didn't do the wrapper trick, we'd have to define each of those `xxxParams()` objects separately and pass it into each constructor call.

Remember, `autocxx` does not necessarily define a `Document::new` for us!


### Solution

* Add `autocxx`, `build.rs`, `example.csv`, `rapidcsv.h` or look at our `rust-exercises/exercises-template/cpp-interop`.

### Write our wrapper.h and print the RowCount

* Write a `open_csv` that will act as our constructor in `wrapper.h`
* Call `doc.GetRowCount();` on your `doc` and print it.

<details><summary> Solution: </summary>
Once you have this in your `src/wrapper.h`:

```cpp
#pragma once // We write this to not trip up the build system!

#include "rapidcsv.h"

namespace my_csv {
    rapidcsv::Document open_csv(const std::string& pPath) {
        return rapidcsv::Document(pPath);
    }
```

We can define our method of interest:

```rust [], ignore
use autocxx::prelude::*;
use cxx::{let_cxx_string, CxxString};
use ffi::rapidcsv::Document;

autocxx::include_cpp! {
    #include "wrapper.h"
    generate!("rapidcsv::Document")
    generate!("my_csv::open_csv")
    safety!(unsafe_ffi)
}

fn main() {
    let_cxx_string!(file_name = "example.csv");
    let doc = ffi::my_csv::open_csv(&file_name).within_unique_ptr();
    let count = doc.GetRowCount();
    println!("{count:?}");
}
```
</details>

And call `doc.getRowCount()` from the Rust side, knowing that all resources will be RAII'd properly.

### Write GetStringCell method

Write an `impl` block to have `doc.get_string_cell(n, i)` work, then print the entries.

We now want to access specific elements to print them:

<details><summary> Solution </summary>

This gives a complete solution to *Task 1*.

We add the method to our `wrapper.h`:

```cpp
#pragma once

#include "rapidcsv.h"

namespace my_csv {
    rapidcsv::Document open_csv(const std::string& pPath) {
        return rapidcsv::Document(pPath);
    }

    std::string get_string_cell(const rapidcsv::Document& doc,const size_t pColumnIdx, const size_t pRowIdx) {
        return doc.GetCell<std::string>(pColumnIdx, pRowIdx);
    }
}
```

```rust [], ignore
use autocxx::prelude::*;
use cxx::{let_cxx_string, CxxString};
use ffi::rapidcsv::Document;

autocxx::include_cpp! {
    #include "wrapper.h"
    generate!("rapidcsv::Document")
    generate!("my_csv::open_csv")
    generate!("my_csv::get_string_cell")
    safety!(unsafe_ffi)
}

fn main() {
    let_cxx_string!(file_name = "example.csv");
    let doc = ffi::my_csv::open_csv(&file_name).within_unique_ptr();
    let count = doc.GetRowCount();
    println!("{count}");
    for i in 0..count {
        let date = doc.get_string_cell(0, i);
        println!("{}", date);
    }
}

trait GetStringCell {
    fn get_string_cell(&self, column: usize, row: usize) -> UniquePtr<CxxString>;
}

impl GetStringCell for Document {
    fn get_string_cell(&self, column: usize, row: usize) -> UniquePtr<CxxString> {
        ffi::my_csv::get_string_cell(self, column, row)
    }
}
```

</details>

### Filter for June dates and process

A solution without iterators:

<details><summary> Solution Task 2 </summary>

```rust [], ignore
use autocxx::prelude::*;
use cxx::{let_cxx_string, CxxString};
use ffi::rapidcsv::Document;

autocxx::include_cpp! {
    #include "wrapper.h"
    generate!("rapidcsv::Document")
    generate!("my_csv::open_csv")
    generate!("my_csv::get_string_cell")
    safety!(unsafe_ffi)
}

fn main() {
    let_cxx_string!(file_name = "weather.csv");
    let doc = ffi::my_csv::open_csv(&file_name).within_unique_ptr();
    let count = doc.GetRowCount();
    let mut june_temps = 0.0;
    for i in 0..count {
        let date = doc.get_string_cell(0, i);
        // Convert to Rust str - with type-guaranteed no additional memory overhead
        if let Ok(date_str) = date.to_str() {
            // Check if it's in June - format MM/DD/YYY
            if date_str.starts_with("6/") {
                // Date is in June, so get the Temp_C value in the 2nd column
                if let Ok(temp) = doc.get_string_cell(1, i).to_str() {
                    if let Ok(temp_float) = temp.parse::<f64>() {
                        june_temps += temp_float;
                    }
                }
            }
        }
    }
    // June always has 30 days
    println!("{}", june_temps / 30.0);
}

trait GetStringCell {
    fn get_string_cell(&self, column: usize, row: usize) -> UniquePtr<CxxString>;
}

impl GetStringCell for Document {
    fn get_string_cell(&self, column: usize, row: usize) -> UniquePtr<CxxString> {
        ffi::my_csv::get_string_cell(self, column, row)
    }
}
```

</details>

### Idiomatic solution

Using Rust iterators:

<details><summary> Idiomatic Solution Task 2 </summary>

```rust [], ignore
use autocxx::prelude::*;
use cxx::{let_cxx_string, CxxString};
use ffi::rapidcsv::Document;

autocxx::include_cpp! {
    #include "wrapper.h"
    generate!("rapidcsv::Document")
    generate!("my_csv::open_csv")
    generate!("my_csv::get_string_cell")
    safety!(unsafe_ffi)
}

fn main() {
    let_cxx_string!(file_name = "weather.csv");
    let doc = ffi::my_csv::open_csv(&file_name).within_unique_ptr();
    let count = doc.GetRowCount();

    let sum_of_june_temperatures: i64 = (0..count)
        .filter_map(|row_index| {
            let date: UniquePtr<CxxString> = doc.get_string_cell(0, row_index);
            let date: &str = date.to_str().ok()?;
            // Check if it's in June: the date format in the file is `M/DD/YYYY`
            date.starts_with("6/").then_some(row_index)
        })
        .filter_map(|row_index| {
            let temperature: UniquePtr<CxxString> = doc.get_string_cell(1, row_index);
            let temperature: &str = temperature.to_str().ok()?;
            temperature.parse::<i64>().ok()
        })
        .sum();
    // June has 30 days
    println!("{}", sum_of_june_temperatures as f64 / 30.0);
```

Explanation:

- Make a multi-step chain to first select the row and second to extract the temperature value
- Mask bad encodings and numerical conversions behind `filter_map`
- We use the `sum` method instead of keeping an explicit accumulation value around

In this particular case getting the nice chain is hard. The `doc.get_string_cell` returns a C++ Unique Pointer, and if you call `to_str()` on it without making an explicit variable first, it will provoke the compiler to drop the temporary `unique_ptr` value (the borrow checker catches it).

This means you can't really `map` or `filter_map` `&str` values from one step to the next. So, keep passing `row_index` instead and do separate extractions in each closure.

`?` is very useful, so use `filter_map` in both steps.

</details>
# Durable File

In this exercise, we will to implement `std` library traits for a `DurableFile`, a type of file which tracks it's syncs and writes.

## After completing this exercise you will be able to

* implement the `Drop` and `Write` traits.
* use the “Drop guard” pattern.
* test panicking APIs.

## Prerequisites

For completing this exercise you need to have:

* an understanding of what `self`, `&self` and `&mut self` means
* knowledge of how to write and wrap a `struct`
* basic knowledge of how to write a `trait`
* the basics on using `Return` and `?`

`std::fs::File` type has a [`sync_all`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.sync_all) method which ensures that all data is written to disk. This method is not called by default: syncing is slow and the OS has good heuristics for optimistically delaying syncing.

In this assignment, we’ll implement a `DurableFile` wrapper for `File`, which helps to ensure that applications calls [`sync_all`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.sync_all). Specifically, `DurableFile` tracks syncs and writes. If a `DurableFile` is dropped with an outstanding write, its `Drop` panics. Not calling [`sync_all`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.sync_all) before disposing the file is considered a bug in this case. A `panic!` helps to elevate silent potential data loss into a loud failure.

## Task

1. Create a new libary crate with `cargo new --lib durable-file`
2. Implement the `DurableFile` data structure and its constructor

```rust
use std::fs::File;

struct DurableFile {
    file: File,
    needs_sync: bool,
}

impl DurableFile {
    pub fn new(file: std::fs::File) -> DurableFile {
        ...
    }
}
```

Optional: implement `From<std::fs::File>` for `DurableFile`

3. Implement the [std::io::Write](https://doc.rust-lang.org/stable/std/io/trait.Write.html) trait for `DurableFile`. Use [sync_all](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.sync_all) in the implementation of the flush method. All write operations should set the `needs_sync` flag, the flush method should clear it.

4. Implement the [std::ops::Drop](https://doc.rust-lang.org/std/ops/trait.Drop.html) trait for `DurableFile` so that it panics if the file is not flushed. What is the right behavior for `Drop`? Are there any edge cases to worry about?

5. Add an inherent `close` method for `DurableFile`, to explicitly sync, dispose the file, and handle potential errors. What is the appropriate signature (type of self and the return type) for close?

6. Write a couple of simple smoke tests. You might find the [tempdir](https://docs.rs/tempdir/0.3.7/tempdir/) crate and [#[should_panic]](https://doc.rust-lang.org/reference/attributes/testing.html#the-should_panic-attribute) attribute useful!

```rust
#[test]
#[should_panic(expected = "not yet implemented")]
fn smoke_test() {
    let dir = tempdir::TempDir::new("tests").unwrap();
    let file = std::fs::File::create(dir.path().join("foo.txt")).unwrap();
    todo!()
}
```

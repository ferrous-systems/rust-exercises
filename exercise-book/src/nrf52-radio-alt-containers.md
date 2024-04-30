# Alternative containers

## Modify-in-place

If you solved the puzzle using a `Vec` buffer you can try solving it without the buffer as a stretch goal. You may find the [slice methods][slice] that let you mutate a `Packet`'s data useful, but remember that the first six bytes of your `Packet` will be the random device address - you can't decrypt those! A solution that does not use a `heapless:Vec` buffer can be found in the `src/bin/radio-puzzle-solution-2.rs` file.

## Using `liballoc::BTreeMap`

If you solved the puzzle using a `heapless::Vec` buffer and a `heapless::LinearMap` and you still need something else to try, you could look at the [`Vec`][vec] and [`BTreeMap`][btreemap] types contained within `liballoc`. This will require you to set up a global memory allocator, like [`embedded-alloc`][embedded-alloc].

[vec]: https://doc.rust-lang.org/alloc/vec/struct.Vec.html
[btreemap]: https://doc.rust-lang.org/alloc/collections/struct.BTreeMap.html
[embedded-alloc]: https://github.com/rust-embedded/embedded-alloc
[slice]: https://doc.rust-lang.org/std/primitive.slice.html#methods

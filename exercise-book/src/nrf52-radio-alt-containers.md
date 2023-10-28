# Alternative containers

## Modify-in-place

If you solved the puzzle using a `Vec` buffer you can try solving it without the buffer as a stretch goal. You may find the [slice methods][slice] that let you mutate its data useful. A solution that does not use a `heapless:Vec` buffer can be found in the `src/bin/radio-puzzle-solution-2.rs` file.

## Using `liballoc::BTreeMap`

If you get all that working and still need something else to try, you could look at the [`BTreeMap`][btreemap] contained within `liballoc`. This will require you to set up a global memory allocator, like [`embedded-alloc`][embedded-alloc].

[btreemap]: https://doc.rust-lang.org/alloc/collections/btree_map/struct.BTreeMap.html
[embedded-alloc]: https://github.com/rust-embedded/embedded-alloc
[slice]: https://doc.rust-lang.org/std/primitive.slice.html#methods

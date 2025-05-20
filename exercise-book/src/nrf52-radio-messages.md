# Messages

In `radio-send.rs` we propose three different ways to define the bytes we want to send to the radio:

```rust
let msg: &[u8; 5] = &[72, 101, 108, 108, 111];
let msg: &[u8; 5] = &[b'H', b'e', b'l', b'l', b'o'];
let msg: &[u8; 5] = b"Hello";
```

Here, we explain the different types.

## Slices

The `send` method takes a *reference* -- in Rust, a reference (`&`) is a non-null pointer that's compile-time known to point into valid (e.g. non-freed) memory --  to a `Packet` as argument. A `Packet` is a stack-allocated, fixed-size buffer. You can fill the `Packet` (buffer) with data using the `copy_from_slice` method -- this will overwrite previously stored data.

This `copy_from_slice` method takes a *slice of bytes* (`&[u8]`). A slice is a reference into a list of elements stored in contiguous memory. One way to create a slice is to take a reference to an *array*, a fixed-size list of elements stored in contiguous memory.

```rust
// stack allocated array
let array: [u8; 3] = [0, 1, 2];

let ref_to_array: &[u8; 3] = &array;
let slice: &[u8] = &array;
```

`slice` and `ref_to_array` are constructed in the same way but have different types. `ref_to_array` is represented in memory as a single pointer (1 word / 4 bytes); `slice` is represented as a pointer + length (2 words, or 8 bytes).

Because slices track length at runtime rather than in their type they can point to chunks of memory of any length.

```rust ignore
let array1: [u8; 3] = [0, 1, 2];
let array2: [u8; 4] = [0, 1, 2, 3];

let mut slice: &[u8] = &array1;
defmt::println!("{:?}", slice); // length = 3

// now point to the other array
slice = &array2;
defmt::println!("{:?}", slice); // length = 4
```

## Byte literals

In the example we sent the list of bytes: `[72, 101, 108, 108, 111]`, which can be interpreted as the string `"Hello"`. To see why this is the case check this [list of printable ASCII characters][ascii]. You'll see that letter `H` is represented by the (single-byte) value `72`, `e` by `101`, etc.

[ascii]: https://en.wikipedia.org/wiki/ASCII#Printable_characters

Rust provides a more convenient way to write ASCII characters: byte literals. `b'H'` is syntactic sugar for the literal `72u8`, `b'e'` is equivalent to `101u8`, etc.. So we can rewrite `[72, 101, 108, 108, 111]` as `[b'H', b'e', b'l', b'l', b'o']`. Note that byte literals can also represent `u8` values that are not printable ASCII characters: those values are written using escaped sequences like `b'\x7F'`, which is equivalent to `0x7F`.

## Byte string literals

`[b'H', b'e', b'l', b'l', b'o']` can be further rewritten as `b"Hello"`. This is called a *byte* string literal (note that unlike a string literal like `"Hello"` this one has a `b` before the opening double quote). A byte string literal is a series of byte literals (`u8` values); these literals have type `&[u8; N]` where `N` is the number of byte literals in the string.

Because byte string literals are references you need to *dereference* them to get an array type.

```rust
let reftoarray: &[u8; 2] = b"Hi";

// these two are equivalent
let array1:  [u8; 2] = [b'H', b'i'];
let array2:  [u8; 2] = *b"Hi";
//          ^          ^ dereference
```

Or if you want to go the other way around: you need to take a reference to an array to get the same type as a byte string literal.

```rust
// these two are equivalent
let reftoarray1: &[u8; 2] = b"Hi";
let reftoarray2: &[u8; 2] = &[b'H', b'i'];
//               ^          ^
```

## Character constraints in byte string vs. string literals

You can encode text as `b"Hello"` or as `"Hello"`.

`b"Hello"` is by definition a string (series) of byte literals so each character has to be a byte literal like `b'A'` or `b'\x7f'`. You cannot use "Unicode characters" (`char` type) like emoji or CJK (Chinese Japanese Korean) in byte string literals.

On the other hand, `"Hello"` is a string literal with type `&str`. `str` strings in Rust contain UTF-8 data so these string literals can contain CJK characters, emoji, Greek letters, Cyrillic script, etc.

## Printing strings and characters

In this exercise we'll work with ASCII strings so byte string literals that contain no escaped characters are OK to use as packet payloads.

You'll note that `defmt::println!("{:?}", b"Hello")` will print `[72, 101, 108, 108, 111]` rather than `"Hello"` and that the `{}` format specifier (`Display`) does not work. This is because the type of the literal is `&[u8; N]` and in Rust this type means "bytes"; those bytes could be ASCII data, UTF-8 data or something else.

To print this you'll need to convert the slice `&[u8]` into a string (`&str`) using the [`core::str::from_utf8`](https://doc.rust-lang.org/stable/core/str/fn.from_utf8.html) function. This function will verify that the slice contains well formed UTF-8 data and interpret it as a UTF-8 string (`&str`). We were careful to ensure that our three example messages were the same, and were all valid UTF-8, so we expect the conversion to always succeed. Why not try and see which bytes cause this conversion to fail?

Something similar will happen with byte literals: `defmt::println!("{}", b'A')` will print `65` rather than `A`. To get the `A` output you can cast the byte literal (`u8` value) to the `char` type: `defmt::println!("{}", b'A' as char)`.

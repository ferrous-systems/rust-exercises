# Help

## Use a dictionary

Our suggestion is to use a dictionary / map. `std::collections::HashMap` is not available in `no_std` code (it requires a secure random number generator to prevent collision attacks) but you can use one of the stack-allocated maps in the [`heapless`] crate. It supplies a stack-allocated, fixed-capacity version of the `std::Vec` type which will come in handy to store byte arrays. To store character mappings we recommend using a `heapless::LinearMap`.

`heapless` is already declared as a dependency in the Cargo.toml of the project so you can directly import it into the application code using a `use` statement.

[`heapless`]: https://docs.rs/heapless

```rust ignore
use heapless::Vec;       // like `std::Vec` but stack-allocated
use heapless::LinearMap; // a dictionary / map

fn main() {
    // A hash map with a capacity of 16 `(u8, u8)` key-value pairs allocated on the stack
    let mut my_map = LinearMap::<u8, u8, 16>::new();
    my_map.insert(b'A', b'~').unwrap();

    // A vector with a fixed capacity of 8 `u8` elements allocated on the stack
    let mut my_vec = Vec::<u8, 8>::new();
    my_vec.push(b'A').unwrap();
}
```

If you haven't used a stack-allocated collection before note that you'll need to
specify the capacity of the collection as a const-generic parameter. The larger
the value, the more memory the collection takes up on the stack. The
[`heapless::LinearMap` documentation][indexMap] of the `heapless` crate has some
usage examples, as does the [`heapless::Vec` documentation][vec].

[indexMap]: https://docs.rs/heapless/0.8.0/heapless/struct.LinearMap.html
[vec]: https://docs.rs/heapless/0.8.0/heapless/struct.Vec.html

## Note the difference between character literals and byte literals!

Something you will likely run into while solving this exercise are *character* literals (`'c'`) and *byte* literals (`b'c'`). The former has type [`char`] and represent a single Unicode "scalar value". The latter has type `u8` (1-byte integer) and it's mainly a convenience for getting the value of ASCII characters, for instance `b'A'` is the same as the `65u8` literal.

[`char`]: https://doc.rust-lang.org/std/primitive.char.html

*IMPORTANT* you do not need to use the `str` or `char` API to solve this problem, other than for printing purposes. Work directly with slices of bytes (`[u8]`) and bytes (`u8`); and only convert those to `str` or `char` when you are about to print them.

> Note: The plaintext string is *not* stored in `puzzle-fw` so running `strings` on it will not give you the answer. Nice try.

## Make sure not to flood the log buffer

When you log more messages than can be moved from the probe to the target, the log buffer will get overwritten, resulting in data loss. This can easily happen when you repeatedly poll the dongle and log the result. The quickest solution to this is to wait a short while until you send the next packet so that the logs can be processed in the meantime.

```rust ignore
use core::time::Duration;

#[entry]
fn main() -> ! {

    let mut timer = board.timer;

    for plainletter in 0..=127 {
        /* ... send letter to dongle ... */
        defmt::println!("got response");
        /* ... store output ... */

        timer.wait(Duration::from_millis(20));
    }
}
```

## Recommended Steps

Each step is demonstrated in a separate example so if for example you only need a quick reference of how to use the map API you can look at step / example number 2 and ignore the others.

1. Send a one letter packet (e.g. `A`) to the radio to get a feel for how the mapping works. Then do a few more letters. See `src/bin/radio-puzzle-1.rs`.

2. Get familiar with the dictionary API. Do some insertions and look ups. What happens if the dictionary gets full? See `src/bin/radio-puzzle-2.rs`.

3. Next, get mappings from the radio and insert them into the dictionary. See `src/bin/radio-puzzle-3.rs`.

4. You'll probably want a buffer to place the plaintext in. We suggest using `heapless::Vec` for this. See `src/bin/radio-puzzle-4.rs` for a starting-point (NB It is also possible to decrypt the packet in place).

5. Simulate decryption: fetch the encrypted string and "process" each of its bytes. See `src/bin/radio-puzzle-5.rs`.

6. Now merge steps 3 and 5: build a dictionary, retrieve the secret string and do the reverse mapping to decrypt the message. See `src/bin/radio-puzzle-6.rs`.

7. As a final step, send the decrypted string to the Dongle and check if it was correct or not. See `src/bin/radio-puzzle-7.rs`.

For your reference, we have provided a complete solution in the `src/bin/radio-puzzle-solution.rs` file. That solution is based on the seven steps outlined above. Did you solve the puzzle in a different way?

All finished? See the [next steps](nrf52-radio-next-steps.md).

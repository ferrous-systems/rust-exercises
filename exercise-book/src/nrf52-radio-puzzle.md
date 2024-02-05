# Radio Puzzle

![illustration showing that you send plaintext and the dongle responds with ciphertext](img/puzzle_illustration.jpg)

Your task in this section is to decrypt the [substitution cipher] encrypted *ASCII* string stored in the Dongle using one of the stack-allocated maps in the [`heapless`] crate. The string has been encrypted using *simple substitution*.

## Preparing the Dongle

[substitution cipher]: https://en.wikipedia.org/wiki/Substitution_cipher
[`heapless`]: https://docs.rs/heapless

✅ Flash the `puzzle-fw` program on the Dongle. Follow the instructions from the "nRF52840 Dongle" section but flash the `puzzle-fw` program instead of the `loopback-fw` one -- don't forget to put the Dongle in bootloader mode (pressing the reset button) before invoking `nrfdfu`.

> Note: If you experienced USB issues with `loopback-fw` you can use the older `puzzle-nousb*.hex` variants.

Like in the previous sections the Dongle will listen for radio packets -- this time over *channel 25* -- while also logging messages over a USB/serial interface. It also prints a `.` periodically so you know it's still alive.

## Sending Messages and Receiving the Dongle's Responses

✅ Open the `nrf52-code/radio-app` folder in VS Code; then open the `src/bin/radio-puzzle.rs` file. Run the program.

This will send a zero sized packet `let msg = b""` to the dongle.

❗ The Dongle responds to the DK's requests wirelessly (i.e. by sending back radio packets) as well. You'll see the dongle responses printed by the DK. This means you don't have to worry if serial-term doesn't work on your machine.

✅ Try sending one-byte sized packets.
✅ Try sending longer packets.

What happens?

❗ The Dongle responds to the DK's requests wirelessly (i.e. by sending back radio packets) as well. You'll see the dongle responses printed by the DK. This means you don't have to worry if serial-term doesn't work on your machine.

<details>
    <summary>Answer</summary>

The Dongle will respond differently depending on the length of the incoming packet:

- On zero-sized packets it will respond with the encrypted string.
- On one-byte sized packets it will respond with the *direct* mapping from a *plaintext* letter (single `u8` value) -- the letter contained in the packet -- to the *ciphertext* letter (`u8` value).
- On packets of any other length the Dongle will respond with the string `correct` if it received the decrypted string, otherwise it will respond with the `incorrect` string.

The Dongle will always respond with packets that are valid UTF-8 so you can use `str::from_utf8` on the response packets.

This step is illustrated in `src/bin/radio-puzzle-1.rs`

</details>

From here on, the exercise can be solved in multiple ways. If you have an idea on how to go from here and what tools to use, you can work on your own. If you don't have an idea what to do next or what tools to use, we'll provide a guide on the next page.

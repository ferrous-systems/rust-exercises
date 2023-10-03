# `dongle`

Pre-made applications for the nRF52840 Dongle.

These applications will be used in the beginner workshop.

## Hardware

### LEDs

- The green LED (LD1) is connected to pin P0.6
- The red channel of the RGB LED (LD2) is connected to pin P0.8
- The green channel of the RGB LED (LD2) is connected to pin P**1**.9
- The blue channel of the RGB LED (LD2) is connected to pin P0.12

Both LEDs are mounted near the USB connector.

### Buttons

- The Reset button is mounted sideways near the edge of the board that's opposite of the USB connector.
- The SW1 button is connected to pin P**1**.06. This round-ish button is right next to the RESET button but closer to the USB connector.

## Changing the puzzle secret string

### Create the secret string

Run the `puzzlegen.rs` program on the host to create a new secret string.

``` console
cargo new --bin puzzlegen
cp puzzlegen.rs puzzlegen/src/main.rs
cd puzzlegen
# or manually modify the Cargo.toml
cargo add rand
# update the plaintext in `src/main.rs`
cargo run
```

Take note of the output; it will look like this:

``` text
from: [116, 68, 123, 97, 47, 46, 90, 120, 34, 49, 59, 39, 50, 106, 71, 75, 108, 115, 81, 117, 69, 57, 76, 41, 100, 38, 93, 58, 78, 126, 70, 56, 84, 111, 113, 91, 89, 55, 40, 114, 122, 52, 61, 64, 45, 79, 67, 83, 48, 66, 63, 104, 43, 77, 44, 54, 98, 92, 94, 60, 62, 118, 87, 80, 95, 74, 65, 112, 109, 73, 110, 101, 53, 86, 33, 121, 42, 35, 85, 82, 105, 36, 37, 119, 125, 51, 96, 99, 88, 32, 103, 72, 107, 124, 102]
to: [81, 78, 109, 61, 120, 87, 125, 98, 100, 91, 97, 66, 57, 117, 49, 64, 48, 85, 75, 73, 92, 101, 83, 110, 62, 89, 35, 37, 93, 71, 123, 121, 60, 38, 115, 102, 59, 47, 108, 80, 58, 44, 86, 111, 41, 84, 96, 50, 51, 70, 43, 112, 79, 46, 113, 107, 106, 116, 65, 68, 69, 77, 105, 56, 103, 67, 40, 54, 99, 55, 45, 63, 34, 88, 119, 74, 94, 32, 114, 36, 95, 124, 118, 76, 126, 72, 82, 122, 33, 104, 90, 39, 42, 52, 53]
secret: "<p=-*Uh5&Ph6=PQ_z_6=Q_-Zh_-h&IPh?cj?>>?>h$IUQhL&P*Up&6w"
```

### Generate `puzzle` ELF

``` console
git clone --branch dongle-puzzle https://github.com/japaric/embedded2020
cd embedded2020/firmware/apps
```

Find `puzzle.rs` in the `embedded2020/firmware/apps/src/bin` folder.

Update `puzzle.rs` with the `FROM`, `TO` and `SECRET` data that you got from `puzzlegen`

```` rust
static FROM: &[u8] = &[
    116, 68, 123, 97, 47, 46, 90, 120, 34, 49, 59, 39, 50, 106, 71, 75, 108, 115, 81, 117, 69, 57,
    76, 41, 100, 38, 93, 58, 78, 126, 70, 56, 84, 111, 113, 91, 89, 55, 40, 114, 122, 52, 61, 64,
    45, 79, 67, 83, 48, 66, 63, 104, 43, 77, 44, 54, 98, 92, 94, 60, 62, 118, 87, 80, 95, 74, 65,
    112, 109, 73, 110, 101, 53, 86, 33, 121, 42, 35, 85, 82, 105, 36, 37, 119, 125, 51, 96, 99, 88,
    32, 103, 72, 107, 124, 102,
];

static TO: &[u8] = &[
    81, 78, 109, 61, 120, 87, 125, 98, 100, 91, 97, 66, 57, 117, 49, 64, 48, 85, 75, 73, 92, 101,
    83, 110, 62, 89, 35, 37, 93, 71, 123, 121, 60, 38, 115, 102, 59, 47, 108, 80, 58, 44, 86, 111,
    41, 84, 96, 50, 51, 70, 43, 112, 79, 46, 113, 107, 106, 116, 65, 68, 69, 77, 105, 56, 103, 67,
    40, 54, 99, 55, 45, 63, 34, 88, 119, 74, 94, 32, 114, 36, 95, 124, 118, 76, 126, 72, 82, 122,
    33, 104, 90, 39, 42, 52, 53,
];

// store the secret rather than the plaintext -- otherwise `strings $elf` will reveal the answer
static SECRET: &[u8] = b"<p=-*Uh5&Ph6=PQ_z_6=Q_-Zh_-h&IPh?cj?>>?>h$IUQhL&P*Up&6w";
````

Build the program; this will produce an ELF file called `puzzle` (no file ending).

``` console
cargo build --bin puzzle --release
```

Copy this ELF file from `embedded2020/firmware/target/thumbv7em-none-eabi/release` to `embedded-trainings-2020/boards/dongle`

Test the produced `puzzle` file:

- flash it onto a dongle using `nrfdfu puzzle`. The green LED on the dongle should turn on
- run `cargo xtask serial-term`; you should see the following output. `deviceid` will be different

    ```text
    deviceid=d90eedf1978d5fd2 channel=25 TxPower=+8dBm app=puzzle
    ```

- run the `radio-puzzle-solution` program on a DK; it should be able to decrypt the new secret
- run `cargo xtask change-channel <some number between 11 and 26>` to test changing the Dongle's radio channel
- modify and re-run the `radio-puzzle-solution` program on a DK to solve the puzzle using a the channel you set in the previous step

### Generate `puzzle-nousb-*`

The procedure is similar to the one for generating the `puzzle` ELF file. The differences are:

- you build `puzzle-nousb.rs` in the `embedded2020` repository and copy `embedded2020/firmware/target/thumbv7em-none-eabi/release/puzzle-nousb` over
- you also need to change `const CHANNEL` in `puzzle-nousb.rs`
- you need to produce one ELF file per hard-coded radio channel.

Also test these `nousb` ELF files. Note that the green LED won't turn on when the dongle restarts! The green LED will toggle when a new packet is received and the blue LED will turn on when the decoded secret is received. Also, `cargo xtask change-channel` won't work with the `nousb` variants so you can skip that test.

## References

- [nRF52840 Dongle section on Nordic Semiconductor's info center](https://infocenter.nordicsemi.com/index.jsp?topic=%2Fug_getting_started%2FUG%2Fgs%2Fdevelop_sw.html&cp=1_0_2)

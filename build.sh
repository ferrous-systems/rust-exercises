#!/usr/bin/env bash

set -euo pipefail

OUTPUT_NAME=${1:-./output}

# Build and test the solutions
pushd exercise-solutions
cargo test --locked
cargo test --examples --locked
cargo fmt --check
pushd connected-mailbox
cargo test --locked
cargo fmt --check
popd
pushd multi-threaded-mailbox
cargo test --locked
cargo fmt --check
popd
popd
pushd qemu-code
pushd uart-driver
# Build from source because armv8r-none-eabihf isn't Tier 2
RUSTC_BOOTSTRAP=1 cargo build -Zbuild-std=core --locked
popd
popd
pushd nrf52-code
pushd boards/dk
cargo build --target=thumbv7em-none-eabihf --locked --release
cargo fmt --check
popd
pushd boards/dk-solution
cargo build --target=thumbv7em-none-eabihf --locked --release
cargo fmt --check
popd
pushd boards/dongle
cargo build --target=thumbv7em-none-eabihf --locked --release
cargo fmt --check
popd
pushd radio-app
cargo build --target=thumbv7em-none-eabihf --locked --release
cargo fmt --check
popd
for i in usb-lib-solutions/*; do
    pushd $i
    cargo build --target=thumbv7em-none-eabihf --locked --release
    cargo fmt --check
    cargo test --locked
    popd
done
pushd usb-lib 
    cargo build --target=thumbv7em-none-eabihf --release --locked
    cargo fmt --check
popd
pushd usb-app
cargo build --target=thumbv7em-none-eabihf --release --locked
cargo fmt --check
popd
pushd usb-app-solutions
cargo build --target=thumbv7em-none-eabihf --release --locked
cargo fmt --check
popd
pushd consts
cargo build --locked
cargo fmt --check
popd
pushd puzzle-fw
cargo build --target=thumbv7em-none-eabihf --release --locked
cargo fmt --check
popd
pushd loopback-fw
cargo build --target=thumbv7em-none-eabihf --release --locked
cargo fmt --check
popd
popd

# Only check the templates (they will panic at run-time due to the use of todo!)
pushd exercise-templates
cargo check --locked
cargo fmt --check
popd

pushd exercise-book
mdbook test
mdbook build
popd

rm -rf "${OUTPUT_NAME}"
mkdir -p "${OUTPUT_NAME}"
mkdir -p "${OUTPUT_NAME}/exercise-book"
# Note: the use of the html subdirectory here is deliberate.
# a) it allows the book to be provided as PDF in the future
# b) it ensures the `../../exercise-solutions` links in the markdown also work
#    when loaded from this output folder. The `../..` comes about
#    because the Markdown book source lives in the `src` subfolder and so you
#    have to go up one extra level. Adding an extra level in the output
#    is easier than re-writing all the links at build time.
mv ./exercise-book/book "${OUTPUT_NAME}/exercise-book/html"
cp -r ./exercise-templates "${OUTPUT_NAME}/"
cp -r ./exercise-solutions "${OUTPUT_NAME}/"
cp -r ./nrf52-code "${OUTPUT_NAME}/"
cp -r ./qemu-code "${OUTPUT_NAME}/"
cp -r ./xtask "${OUTPUT_NAME}/"
cp -r ./.cargo "${OUTPUT_NAME}/"
cp -r ./tools "${OUTPUT_NAME}/"
cp ./nrf52-code/puzzle-fw/target/thumbv7em-none-eabihf/release/puzzle-fw "${OUTPUT_NAME}/nrf52-code/boards/dongle-fw/puzzle-fw"
cp ./nrf52-code/loopback-fw/target/thumbv7em-none-eabihf/release/loopback-fw "${OUTPUT_NAME}/nrf52-code/boards/dongle-fw/loopback-fw"
find "${OUTPUT_NAME}" -name target -type d -print0 | xargs -0 rm -rf
zip -r "${OUTPUT_NAME}.zip" "${OUTPUT_NAME}"

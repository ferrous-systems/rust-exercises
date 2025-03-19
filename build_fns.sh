#!/bin/bash

#
# Ferrous Systems CI deployment setup
#
# Copyright (c) Ferrous Systems, 2025
#
# This script will define functions for testing this repo.
#
#set -euo pipefail

# Build and test the solutions
# exercise-solutions
function test_examples() {
	cd "$1" || return 1
	cargo test --locked
	cargo test --examples --locked
	return 0
}

# exercise-solutions/connected-mailbox
# exercise-solutions/multi-threaded-mailbox
function test() {
	cd "$1" || return 1
	cargo test --locked
	return 0
}

# qemu-code/uart-driver
function build_core() {
	cd "$1" || return 1
	RUSTC_BOOTSTRAP=1 cargo build -Zbuild-std=core --locked
	return 0
}

# nrf52-code/boards/dk
# nrf52-code/boards/dk-solution
# nrf52-code/boards/dongle
# nrf52-code/boards/radio-app
# nrf52-code/usb-app
# nrf52-code/usb-app-solutions
# nrf52-code/consts
# nrf52-code/puzzle-fw
# nrf52-code/loopback-fw
function build_thumbv7em() {
	cd "$1" || return 1
	cargo build --target=thumbv7em-none-eabihf --locked --release
	return 0
}

# nrf52-code/usb-lib-solutions/complete
# nrf52-code/usb-lib-solutions/get-descriptor-config
# nrf52-code/usb-lib-solutions/get-device
# nrf52-code/usb-lib-solutions/set-config
function build_test_thumbv7em() {
	cd "$1" || return 1
	cargo build --target=thumbv7em-none-eabihf --locked --release
	cargo test --locked
	return 0
}

# exercise-templates
function check_templates() {
	cd "$1" || return 1
	cargo check --locked
	return 0
}

function mdbook_test_build() {
	cd "$1" || return 1
	mdbook test
	mdbook build
	return 0
}	

function zip_output() {
	OUTPUT_NAME=${1:-./output}
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
}


FAILURE=0

fail() {
	echo "********************************************************"
    echo "cargo fmt failed: dir $1 did not pass 'cargo fmt --check'"
    echo "********************************************************"
	FAILURE=1
}

function check_fmt() {
dirs=("exercise-solutions"
	"exercise-solutions/connected-mailbox"
	"exercise-solutions/multi-threaded-mailbox"
	"qemu-code/uart-driver"
	"nrf52-code/boards/dk"
	"nrf52-code/boards/dk-solution"
	"nrf52-code/boards/dongle"
	"nrf52-code/radio-app"
	"nrf52-code/usb-app"
	"nrf52-code/usb-app-solutions"
	"nrf52-code/consts"
	"nrf52-code/puzzle-fw"
	"nrf52-code/loopback-fw"
	"nrf52-code/usb-lib-solutions/complete"
	"nrf52-code/usb-lib-solutions/get-descriptor-config"
	"nrf52-code/usb-lib-solutions/get-device"
	"nrf52-code/usb-lib-solutions/set-config"
	"exercise-templates"
)


for dir in "${dirs[@]}"; do
	pushd "$dir" || exit
	cargo fmt --check || fail "$dir"
	popd || exit
done

if [ "$FAILURE" == "1" ]; then
    echo "***************************************************"
    echo "          ❌ cargo fmt has failed! ❌"
    echo "***************************************************"
    exit 1
else
    echo "***************************************************"
    echo "          ✅ cargo fmt has passed! ✅ "
    echo "***************************************************"
fi
}

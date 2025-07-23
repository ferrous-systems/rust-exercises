# Justfile for the Ferrous Systems Rust Exercises
#
# Copyright (c) Ferrous Systems, 2025

set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

package_list := " \
	exercise-solutions \
	exercise-solutions/connected-mailbox\
	exercise-solutions/multi-threaded-mailbox\
	exercise-templates \
	qemu-code/uart-driver\
	nrf52-code/boards/dk \
	nrf52-code/boards/dk-solution \
	nrf52-code/boards/dongle \
	nrf52-code/radio-app \
	nrf52-code/usb-app \
	nrf52-code/usb-app-solutions \
	nrf52-code/consts \
	nrf52-code/puzzle-fw \
	nrf52-code/loopback-fw \
	nrf52-code/usb-lib-solutions/complete \
	nrf52-code/usb-lib-solutions/get-descriptor-config \
	nrf52-code/usb-lib-solutions/get-device \
	nrf52-code/usb-lib-solutions/set-config \
	xtask \
	tools/tcp-client \
"

default:
  @just --choose

everything: test-mdbook build-mdbook test-exercise-templates test-exercise-solutions test-connected-mailbox test-multi-threaded-mailbox build-qemu-uart-driver build-radio-app build-usb-app test-usb-lib build-puzzle-fw build-loopback-fw format

format-check: format-check-rust

format: format-rust

clean: clean-rust
	rm -rf ./training-book/book

serve:
	cd training-book && mdbook serve

build-mdbook:
	cd exercise-book && RUST_LOG=info mdbook build

test-mdbook:
	cd exercise-book && RUST_LOG=info mdbook test

test-exercise-templates:
		cd exercise-templates && cargo build
		cd exercise-templates && cargo test

test-exercise-solutions:
		cd exercise-solutions && cargo build
		cd exercise-solutions && cargo test

test-connected-mailbox:
		cd exercise-solutions/connected-mailbox && cargo build
		cd exercise-solutions/connected-mailbox && cargo test

test-multi-threaded-mailbox:
		cd exercise-solutions/multi-threaded-mailbox && cargo build
		cd exercise-solutions/multi-threaded-mailbox && cargo test

build-qemu-uart-driver:
	cd qemu-code/uart-driver && RUSTC_BOOTSTRAP=1 cargo build --release -Zbuild-std=core

build-radio-app:
	cd nrf52-code/radio-app && cargo build --release

build-usb-app:
	cd nrf52-code/usb-app && cargo build --release
	cd nrf52-code/usb-app-solutions && cargo build --release

test-usb-lib:
	cd nrf52-code/usb-lib && cargo build --release
	cd nrf52-code/usb-lib-solutions/complete && cargo build --release
	cd nrf52-code/usb-lib-solutions/get-descriptor-config && cargo build --release
	cd nrf52-code/usb-lib-solutions/get-device && cargo build --release
	cd nrf52-code/usb-lib-solutions/set-config && cargo build --release

build-puzzle-fw:
	cd nrf52-code/puzzle-fw && cargo build --release

build-loopback-fw:
	cd nrf52-code/loopback-fw && cargo build --release

build-nrf52-code: build-radio-app build-usb-app test-usb-lib build-puzzle-fw build-loopback-fw

assemble version:
	echo "Making ./rust-exercises-{{ version }}..."
	rm -rf ./rust-exercises-{{ version }}
	mkdir -p ./rust-exercises-{{ version }}/exercise-book
	mv ./exercise-book/book ./rust-exercises-{{ version }}/exercise-book/html
	cp -r ./exercise-templates ./rust-exercises-{{ version }}
	cp -r ./exercise-solutions ./rust-exercises-{{ version }}
	cp -r ./nrf52-code ./rust-exercises-{{ version }}
	cp -r ./qemu-code ./rust-exercises-{{ version }}
	cp -r ./xtask ./rust-exercises-{{ version }}
	cp -r ./.cargo ./rust-exercises-{{ version }}
	cp -r ./tools ./rust-exercises-{{ version }}
	cp ./nrf52-code/puzzle-fw/target/thumbv7em-none-eabihf/release/puzzle-fw "./rust-exercises-{{ version }}/nrf52-code/boards/dongle-fw/puzzle-fw"
	cp ./nrf52-code/loopback-fw/target/thumbv7em-none-eabihf/release/loopback-fw "./rust-exercises-{{ version }}/nrf52-code/boards/dongle-fw/loopback-fw"
	echo "Compressing ./rust-exercises-{{ version }}.zip..."
	zip -r ./rust-exercises-{{ version }}.zip ./rust-exercises-{{ version }}

format-check-rust:
	#!/bin/sh
	FAIL=0
	for package in {{ package_list }}; do
		echo "Checking ${package}..."
		( cd ${package} && cargo fmt --check ) || FAIL=1
	done
	if [[ "$FAIL" == 1 ]]; then exit 1; else echo "Formatting all OK"; fi

format-rust:
	#!/bin/sh
	for package in {{ package_list }}; do
		echo "Formatting ${package}..."
		( cd ${package} && cargo fmt )
	done

clean-rust:
	#!/bin/sh
	for package in {{ package_list }}; do
		echo "Cleaning ${package}..."
		( cd ${package} && cargo clean )
	done

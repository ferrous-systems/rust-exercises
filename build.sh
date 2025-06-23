#!/usr/bin/env bash

set -euo pipefail


# This file is a helper file to help build and test local builds of this repository.

# Load all build functions
# All of the build/testing logic should be defined inside build_fns.sh
. ./build_fns.sh


# Check the formatting
check_fmt

# `rapidcsv.h` is a 3rd party library, so we have to download it
curl -o exercise-solutions/cpp-interop/src/rapidcsv.h https://raw.githubusercontent.com/d99kris/rapidcsv/a98b85e663114b8fdc9c0dc03abf22c296f38241/src/rapidcsv.h
cp exercise-solutions/cpp-interop/src/rapidcsv.h exercise-templates/cpp-interop/src/rapidcsv.h

# Build and test the solutions
pushd exercise-solutions
test_examples
pushd connected-mailbox
test_standalone
popd
pushd multi-threaded-mailbox
test_standalone
popd
popd

# Build from source because armv8r-none-eabihf isn't Tier 2
pushd qemu-code
pushd uart-driver
build_qemu_core
popd
popd

pushd nrf52-code
pushd boards/dk
build_thumbv7em
popd
pushd boards/dk-solution
build_thumbv7em
popd
pushd boards/dongle
build_thumbv7em
popd
pushd radio-app
build_thumbv7em
popd
for i in usb-lib-solutions/*; do
    pushd "$i"
    build_test_thumbv7em
    popd
done
pushd usb-lib 
build_thumbv7em
popd
pushd usb-app
build_thumbv7em
popd
pushd usb-app-solutions
build_thumbv7em
popd
pushd consts
build_thumbv7em
popd
pushd puzzle-fw
build_thumbv7em
popd
pushd loopback-fw
build_thumbv7em
popd
popd

# Only check the templates (they will panic at run-time due to the use of todo!)
pushd exercise-templates
check_templates
popd

# Build and test the mdbook build
pushd exercise-book
mdbook_test_build
popd

# Zip the output
zip_output
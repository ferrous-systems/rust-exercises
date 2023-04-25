#!/bin/bash

set -euo pipefail

# Build and test the solutions
pushd exercise-solutions
cargo test
cargo fmt --check
popd

# Only build the templates (they will panic at run-time due to the use of todo!)
pushd exercise-templates
cargo build
cargo fmt --check
popd

pushd exercise-book
mdbook test
mdbook build
popd

rm -rf ./output
mkdir -p ./output
mkdir -p ./output/exercise-book
# Note: the use of the html subdirectory here is deliberate.
# a) it allows the book to be provided as PDF in the future
# b) it ensures the `../../exercise-solutions` links in the markdown also work
#    when loaded from this output folder. The `../..` comes about
#    because the Markdown book source lives in the `src` subfolder and so you
#    have to go up one extra level. Adding an extra level in the output
#    is easier than re-writing all the links at build time.
mv ./exercise-book/book ./output/exercise-book/html
cp -r ./exercise-templates ./output/
cp -r ./exercise-solutions ./output/
zip -r ./output.zip ./output

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

mkdir -p ./output
mkdir -p ./output/exercise-book
mv ./exercise-book/book ./output/exercise-book/html
cp -r ./exercise-templates ./output/
cp -r ./exercise-solutions ./output/
zip -r ./output.zip ./output

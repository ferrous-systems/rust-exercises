#!/bin/bash

#
# Ferrous Systems Cloudflare Deployment Script
#
# Copyright (c) Ferrous Systems, 2026
#
# This script will find every tag in this repo and build the material.

set -euo pipefail

# We only support macOS (the x86 binaries work OK on Apple Silicon), or x86-64 Linux
if [ "$(uname)" == "Darwin" ]; then
    ./mdbook --version || curl -sSL https://github.com/rust-lang/mdBook/releases/download/v0.5.2/mdbook-v0.5.2-x86_64-apple-darwin.tar.gz | tar -xvzf -
    dot -V || brew install graphviz
    mdbook-graphviz --version || cargo install mdbook-graphviz --locked
else
    ./mdbook --version || curl -sSL https://github.com/rust-lang/mdBook/releases/download/v0.5.2/mdbook-v0.5.2-x86_64-unknown-linux-gnu.tar.gz | tar -xvzf -
    dot -V || ( curl -ssL https://github.com/restruct/dot-static/raw/refs/heads/master/x64/dot_static -o ./dot && chmod a+x ./dot )
    ./mdbook-graphviz --version || ( curl -sSL https://github.com/dylanowen/mdbook-graphviz/releases/download/v0.3.1/mdbook-graphviz_v0.3.1_x86_64-unknown-linux-musl.zip -o mdbook-graphviz.zip \
        && unzip mdbook-graphviz.zip \
        && rm mdbook-graphviz.zip \
        && chmod a+x ./mdbook-graphviz )
fi

# Must be an absolute path, otherwise mdbook puts the output in the wrong place
OUTPUT_DIR=$(pwd)/html
CODE_FOLDER_LIST="exercise-solutions exercise-templates nrf52-code qemu-code"
VERSION_FILE="${OUTPUT_DIR}/history/index.html"

# Clean out whatever exists, and make the index (before we do any git checkouts
# and lose the files from this checkout)
rm -rf "${OUTPUT_DIR}"
mkdir -p "${OUTPUT_DIR}"
mkdir -p "${OUTPUT_DIR}/history"
cp ./_redirects "${OUTPUT_DIR}/_redirects"
cp ./index-top.html "${VERSION_FILE}"

# Build the book and slides
function build_and_store {
    mkdir -p "${OUTPUT_DIR}/$1"
    # Munge all the relative source code links to point at Github
    for folder in ${CODE_FOLDER_LIST}; do
        echo "Processing ${folder}"
        # This finds a Markdown links with relative URLs
        find ./exercise-book -type f -name "*.md" -exec sed -e "s~(../../${folder}/~(https://github.com/ferrous-systems/rust-exercises/tree/$2/${folder}/~g" -i.backup {} \;
        # This finds a Markdown references with relative URLs
        find ./exercise-book -type f -name "*.md" -exec sed -e "s~]: ../../${folder}/~]: https://github.com/ferrous-systems/rust-exercises/tree/$2/${folder}/~g" -i.backup {} \;
    done
    # Build the book first, because mdbook will create any empty sections
    # The PATH override lets it find our local copy of mdbook-graphviz
    PATH=$PATH:$(pwd) ./mdbook build -d "${OUTPUT_DIR}/$1/book" ./exercise-book
}

# What branch are we building first?
git_branch=$(git branch --show-current)
current_branch="${CF_PAGES_BRANCH:-$git_branch}"

if [ "$current_branch" == "" ]; then
    echo "Current branch unknown"
    exit 1
fi

# Build what we currently have checked out
# Use what CloudFlare thinks is the current branch, or what git thinks
build_and_store latest "${current_branch}"

# Fetch all the git tags (in case this is some kind of shallow clone)
git fetch --tags


for tag in $(git tag | sort -V); do
    url1="https://github.com/ferrous-systems/rust-exercises/releases/download/${tag}/rust-exercises-${tag}.zip"
    url2="https://github.com/ferrous-systems/rust-exercises/releases/download/${tag}/output.zip"
    echo "Unpacking ${tag}..."
    if [ ! -f "${tag}.zip" ]; then
        curl -sfSL "${url1}" -o "${tag}.zip" || curl -sfSL "${url2}" -o "${tag}.zip"
    fi
    # Make a place to put the rendered output
    mkdir -p "${OUTPUT_DIR}/${tag}"
    unzip -q "${tag}.zip" -d "${OUTPUT_DIR}/${tag}"
    mv "${OUTPUT_DIR}/${tag}"/*/exercise-book/html "${OUTPUT_DIR}/${tag}/book"
    for folder in ${CODE_FOLDER_LIST}; do
        echo "Processing ${folder}"
        # This finds HTML links with relative URLs
        find "${OUTPUT_DIR}/${tag}/book" -type f -name "*.html" -exec sed -e "s~\"../../${folder}/~\"https://github.com/ferrous-systems/rust-exercises/tree/${tag}/${folder}/~g" -i.backup {} \;
    done
    rm -rf "${OUTPUT_DIR}/${tag}"/rust-*
    rm -rf "${OUTPUT_DIR}/${tag}"/output
    echo "<li><a href=\"/${tag}/book\">${tag}</a></li>" >> "${VERSION_FILE}"
done

cat ./index-bottom.html >> "${VERSION_FILE}"

#!/bin/bash

#
# Ferrous Systems Cloudflare Deployment Script
#
# Copyright (c) Ferrous Systems, 2024
#
# This script will find every tag in this repo and build the material.

set -euo pipefail

# We only support macOS (the x86 binaries work OK on Apple Silicon), or x86-64 Linux
if [ $(uname) == "Darwin" ]; then
    ./mdbook --version || curl -sSL https://github.com/rust-lang/mdBook/releases/download/v0.4.40/mdbook-v0.4.40-x86_64-apple-darwin.tar.gz | tar -xvzf -
    ./mdbook-mermaid --version || curl -sSL https://github.com/badboy/mdbook-mermaid/releases/download/v0.13.0/mdbook-mermaid-v0.13.0-x86_64-apple-darwin.tar.gz | tar -xvzf -
else
    ./mdbook --version || curl -sSL https://github.com/rust-lang/mdBook/releases/download/v0.4.40/mdbook-v0.4.40-x86_64-unknown-linux-gnu.tar.gz | tar -xvzf -
    ./mdbook-mermaid --version || curl -sSL https://github.com/badboy/mdbook-mermaid/releases/download/v0.13.0/mdbook-mermaid-v0.13.0-x86_64-unknown-linux-gnu.tar.gz | tar -xvzf -
fi

# Must be an absolute path, otherwise mdbook puts the output in the wrong place
OUTPUT_DIR=$(pwd)/html
CODE_FOLDER_LIST="exercise-solutions exercise-templates nrf52-code qemu-code"

# Clean out whatever exists, and make the index (before we do any git checkouts
# and lose the files from this checkout)
rm -rf "${OUTPUT_DIR}"
mkdir -p "${OUTPUT_DIR}"
cp ./_redirects "${OUTPUT_DIR}/_redirects"

# Build the book and slides
#
# $1 is the output folder name
# $2 is the git tag name
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
    # The PATH override lets it find our local copy of mdbook-mermaid
    PATH=$PATH:. ./mdbook build -d "${OUTPUT_DIR}/$1/book" ./exercise-book
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

for tag in $(git tag); do
    echo "Building ${tag}..."
    # Make a place to put the rendered output
    mkdir -p "${OUTPUT_DIR}/${tag}"
    # Fetch a clean copy of the source material for this tag
    git checkout -f "${tag}"
    # Output dir and git tag are the same
    build_and_store "${tag}" "${tag}"
done

#!/usr/bin/env bash
set -Eeou pipefail

ROOT_DIR=$(git rev-parse --show-toplevel)
RUST_TRIPLE="x86_64-unknown-linux-musl"
PACKAGE_DIR="target/$RUST_TRIPLE/release/"

REPO_OWNER=proctorlabs
REPO_NAME=ecli
GITHUB_API_BASE="https://api.github.com/repos/$REPO_OWNER/$REPO_NAME"

# Build
(
    cd $ROOT_DIR
    rustup target add $RUST_TRIPLE
    cargo build --release --target $RUST_TRIPLE
)

# Create archive
(
    cd $ROOT_DIR/$PACKAGE_DIR
    strip ecli
    tar -cvJf ecli.tar.xz ecli
)

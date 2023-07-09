#!/bin/bash

set -e
cd $(dirname -- "$0"; )
cd ../..

echo "[ci] Starting local CI 'all targets'..."

echo "[ci] Building..."
cargo build

echo "[ci] Formatting..."
cargo fmt --check

echo "[ci] Linting..."
cargo clippy --all-targets --all-features -- -D warnings

echo "[ci] Testing..."
cargo test

echo "[ci] Local CI 'all targets' done!"

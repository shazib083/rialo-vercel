#!/usr/bin/env bash
set -euo pipefail

echo "Installing Rust toolchain..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.org | sh -s -- -y
source "$HOME/.cargo/env"

echo "Building rialo-tester for Linux..."
cargo build --release --manifest-path .rialo-tester/Cargo.toml --target-dir .rialo-tester/target

mkdir -p bin
cp .rialo-tester/target/release/rialo-tester bin/rialo-tester
chmod +x bin/rialo-tester

echo "Build complete: bin/rialo-tester"

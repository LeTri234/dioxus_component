#!/bin/bash
set -e

echo "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

echo "Adding wasm32 target..."
rustup target add wasm32-unknown-unknown

echo "Installing Dioxus CLI..."
cargo install dioxus-cli

echo "Installing Tailwind..."
npm install

echo "Building demo..."
dx build --example demo --release --features web

echo "Build complete!"

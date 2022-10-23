#!/bin/sh

echo ">> Building contract"

rustup target add wasm32-unknown-unknown
cargo build --all --target wasm32-unknown-unknown --release

# RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
# cp target/wasm32-unknown-unknown/release/carbonite_nt_nft.wasm ./res/carbonite_nt_nft.wasm

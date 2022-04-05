#!/bin/sh

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

rustup target add wasm32-unknown-unknown

cd generate-assets
./generate_assets.sh
cd ..

cd generate-errors
./generate_errors.sh
cd ..

cargo install wasm-bindgen-cli
cd generate-wasm-examples
./generate_wasm_examples.sh
cd ..

zola build
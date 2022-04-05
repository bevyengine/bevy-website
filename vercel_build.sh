#!/bin/sh

amazon-linux-extras install rust1
# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# source $HOME/.cargo/env

cd generate-assets
./generate_assets.sh
cd ..

zola build
#!/bin/sh

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -- -y

cd generate-assets
./generate_assets.sh
cd ..

zola build
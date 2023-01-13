#!/bin/sh

git clone --depth=1 https://github.com/bevyengine/bevy-assets assets

cargo run --release --bin generate -- assets ../content

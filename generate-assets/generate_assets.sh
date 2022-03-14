#!/bin/sh

git clone https://github.com/bevyengine/bevy-assets assets

cargo run --bin generate -- assets ../content

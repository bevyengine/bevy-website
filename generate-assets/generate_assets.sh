#!/bin/sh

git clone --branch bevy-asset https://github.com/bevyengine/awesome-bevy assets

cargo run --bin generate -- assets ../content

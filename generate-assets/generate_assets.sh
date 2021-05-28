#!/bin/sh

git clone --branch bevy-asset https://github.com/bevyengine/awesome-bevy assets

cargo run -- assets ../content

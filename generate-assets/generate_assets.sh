#!/bin/sh

git clone --branch migration https://github.com/mockersf/awesome-bevy assets

cargo run -- assets ../content

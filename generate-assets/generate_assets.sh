#!/bin/sh

# Switch to script's directory, letting it be called from any folder.
cd $(dirname $0)

git clone --depth=1 https://github.com/bevyengine/bevy-assets assets

cargo run --release --bin generate -- assets ../content

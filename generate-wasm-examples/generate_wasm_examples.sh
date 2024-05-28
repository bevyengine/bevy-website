#!/usr/bin/env bash

echo Generating WASM example list...

# Switch to script's directory, letting it be called from any folder.
cd $(dirname $0)

# If Bevy folder already exists, pull the latest changes.
if [[ -d bevy ]]; then
    echo Bevy folder already exists, attempting to fetch latest changes.

    cd bevy

    # Attempts to fetch the latest commits, which should only happen every Bevy release.
    git pull --depth=1
else
    echo Bevy folder does not exist, cloning repository.

    # Clone Bevy's latest branch from scratch, only downloading the latest commit.
    git clone --depth=1 --branch=latest https://github.com/bevyengine/bevy bevy

    cd bevy
fi

# Build WebGL2 example list.
rm -rf ../../content/examples
# HACK: build in ./content before moving examples to correct folder in order for `extra.code_path`
# to be formatted correctly.
cargo run -p example-showcase -- build-website-list --content-folder content --api webgl2
mv content ../../content/examples

# Build WebGPU example list.
rm -rf ../../content/examples-webgpu
# HACK: build in ./content before moving examples to correct folder in order for `extra.code_path`
# to be formatted correctly.
cargo run -p example-showcase -- build-website-list --content-folder content --api webgpu
mv content ../../content/examples-webgpu

# Remove Markdown files from assets so that they don't get picked up by Zola.
find assets -type f -name '*.md' -exec rm {} +

# Copy remaining assets to examples folder.
cp -r assets/ ../../static/assets/examples/

echo Finished generating WASM example list! \(WebGL2 + WebGPU\)

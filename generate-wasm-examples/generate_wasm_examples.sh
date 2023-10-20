#!/bin/sh
set -x

# fetch bevy, just for bevyengine/tools
git clone https://github.com/bevyengine/bevy --depth 1
cd bevy

# build examples for webgl2
cargo run -p example-showcase -- build-website-list --content-folder content --api webgl2
mv content ../../content/examples

# build examples for webgpu
cargo run -p example-showcase -- build-website-list --content-folder content --api webgpu
mv content ../../content/examples-webgpu

# remove markdown files from assets so that they don't get picked up by Zola
find assets -type f -name '*.md' -exec rm {} +
cp -r assets/ ../../static/assets/examples/

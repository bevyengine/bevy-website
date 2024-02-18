#!/bin/sh

# Switch to script's directory, letting it be called from any folder.
cd $(dirname $0)

./clone_bevy.sh

# temporary: fetch tools from main branch
git clone --depth=1 https://github.com/bevyengine/bevy bevy-tools

rm -rf bevy/tools
cp -r bevy-tools/tools bevy
rm -rf bevy-tools
cd bevy

cargo run -p example-showcase -- build-website-list --content-folder content --api webgl2
mv content ../../content/examples

rm -rf content

cargo run -p example-showcase -- build-website-list --content-folder content --api webgpu
mv content ../../content/examples-webgpu

# remove markdown files from assets so that they don't get picked up by Zola
find assets -type f -name '*.md' -exec rm {} +
cp -r assets/ ../../static/assets/examples/

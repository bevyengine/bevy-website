#!/bin/sh

# Switch to script's directory, letting it be called from any folder.
cd $(dirname $0)

git init bevy-community
cd bevy-community
git remote add origin https://github.com/bevyengine/bevy-community
git pull --depth=1 origin main
cd ..

cargo run --bin generate -- bevy-community ../content/ community

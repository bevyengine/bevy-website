#!/bin/sh

# Switch to script's directory, letting it be called from any folder.
cd $(dirname $0)

# Download a copy of the Bevy community repository.
git clone --depth=1 https://github.com/bevyengine/bevy-community bevy-community

cargo run --bin generate -- bevy-community ../content/ community

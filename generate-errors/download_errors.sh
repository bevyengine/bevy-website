#!/bin/sh

# Switch to script's directory, letting it be called from any folder.
cd $(dirname $0)

# Only download the `errors` folder from the main Bevy repository.
git clone --no-checkout --depth=1 --filter=tree:0 https://github.com/bevyengine/bevy && cd bevy
git sparse-checkout set --no-cone /errors
git checkout

cd ..

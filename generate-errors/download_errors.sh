#!/bin/sh

# Switch to script's directory, letting it be called from any folder.
cd $(dirname $0)

# Only download the `errors` folder from the main Bevy repository.
git init bevy
cd bevy
git remote add origin https://github.com/bevyengine/bevy
git sparse-checkout set "errors"
git pull --depth=1 origin latest
cd ..
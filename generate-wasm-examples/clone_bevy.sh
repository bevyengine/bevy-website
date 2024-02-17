#!/bin/sh

# Switch to script's directory, letting it be called from any folder.
cd $(dirname $0)

git clone --depth=1 --branch=latest https://github.com/bevyengine/bevy bevy

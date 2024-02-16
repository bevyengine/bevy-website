#!/bin/sh

# Switch to script's directory, letting it be called from any folder.
cd $(dirname $0)

git init bevy
cd bevy
git remote add origin https://github.com/bevyengine/bevy
git pull --depth=1 origin latest

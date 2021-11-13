#!/bin/sh

git init bevy
cd bevy
git remote add origin https://github.com/bevyengine/bevy
git sparse-checkout set "errors"
git pull --depth=1 origin latest
cd ..

cargo run --bin generate -- bevy/errors ../content/learn

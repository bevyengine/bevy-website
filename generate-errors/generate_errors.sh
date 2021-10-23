#!/bin/sh

git init bevy
cd bevy
#git remote add origin https://github.com/bevyengine/bevy
git remote add origin https://github.com/mockersf/bevy
git sparse-checkout set "errors"
#git pull --depth=1 origin main
git pull --depth=1 origin error-codes
cd ..

cargo run --bin generate -- bevy/errors ../content

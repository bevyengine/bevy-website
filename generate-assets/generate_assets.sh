#!/bin/sh

git clone https://github.com/bevyengine/awesome-bevy.git old_assets

python3 parse_old_readme.py old_assets new_assets

cargo run -- new_assets ../content

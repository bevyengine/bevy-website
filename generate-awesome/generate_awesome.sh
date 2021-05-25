#!/bin/sh

git clone https://github.com/bevyengine/awesome-bevy.git old_awesome

python3 parse_old_readme.py old_awesome new_awesome

cargo run -- new_awesome ../content

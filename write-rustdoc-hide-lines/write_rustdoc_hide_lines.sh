#!/bin/sh

# Switch to script's directory, letting it be called from any folder.
cd $(dirname $0)

cargo run --release -- format ../content

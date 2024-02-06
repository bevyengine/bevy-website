#!/bin/sh

# Switch to `write-rustdoc-hide-lines` directory.
cd $(dirname $0)

cargo run --release -- format ../content/learn/book ../content/learn/quick-start

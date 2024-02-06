#!/bin/sh

# Switch to `write-rustdoc-hide-lines` directory.
cd $(dirname $0)

cargo run --release -- ../content/learn/book
cargo run --release -- ../content/learn/quick-start

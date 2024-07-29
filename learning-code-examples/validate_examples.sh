#!/bin/sh

# Switch to script's directory, letting it be called from any folder.
cd $(dirname $0)

cargo check --examples && cargo clippy --examples && cargo fmt --check

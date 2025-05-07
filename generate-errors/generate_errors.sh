#!/bin/sh

# Switch to script's directory, letting it be called from any folder.
cd $(dirname $0)

./download_errors.sh

cargo run --bin generate -- --errors-path bevy/errors --output-path ../content/learn

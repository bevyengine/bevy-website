#!/bin/bash

# locally build wasm examples for testing
# usage:
#   ./build_wasm-examples_debug.sh 6 # builds the first 6 examples

if ! command -v wasm-bindgen >/dev/null 2>&1
then
    echo "wasm-bindgen could not be found - install it using: cargo install wasm-bindgen-cli"
    exit 1
fi

if [ -z "$1" ]
  then
    echo "Missing example amount, use: './build_wasm_examples_debug.sh 5' to build the first 5 examples"
    exit 1
fi

# Switch to script's directory, letting it be called from any folder.
cd $(dirname $0)

# generate_wasm_examples will clone the bevy repo to ./bevy by default.
# If you want to use your fork or local changes, clone, symlink or copy your own bevy repo/folder:
# git clone https://github.com/<username>/bevy bevy
# cp ~/projects/bevy ./bevy
./generate_wasm_examples.sh --no-pull

cd bevy

git reset HEAD --hard
CARGO_PROFILE_RELEASE_OPT_LEVEL='z' CARGO_PROFILE_RELEASE_DEBUG="true" cargo run -p example-showcase -- --per-page $1 --page 0 build-wasm-examples --content-folder ../../static/assets/examples/wasm_webgl2 --api webgl2 --website-hacks
CARGO_PROFILE_RELEASE_OPT_LEVEL='z' CARGO_PROFILE_RELEASE_DEBUG="true" cargo run -p example-showcase -- --per-page $1 --page 0 build-wasm-examples --content-folder ../../static/assets/examples/wasm_webgpu --api webgpu

# remove the examples which this did not build, to make it easier to find which is which
remove_missing() {
    for d in "$1"/*; do
        [[ -d "$d" ]] || continue
        local base=$(basename "$d")
        local match=$(find "$2" -maxdepth 1 -type d -iname "${base/-/?}" | head -n1)
        if [[ -z "$match" ]]; then
            rm -rf "$d"
        else
            remove_missing "$d" "$match"
        fi
    done
}
cd ..
remove_missing "../content/examples" "../static/assets/examples/wasm_webgl2"

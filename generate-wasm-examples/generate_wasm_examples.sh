#!/bin/sh

git init bevy
cd bevy
git remote add origin https://github.com/bevyengine/bevy
git pull --depth=1 origin main

# remove markdown files from assets so that they don't get picked up by Zola
find assets -type f -name '*.md' -exec rm {} +

# setting the default width of a window to 1024
sed -i.bak 's/width: 1280.,/width: 1024.,/' crates/bevy_window/src/window.rs

# setting a canvas by default to help with integration
sed -i.bak 's/canvas: None,/canvas: Some("#bevy".to_string()),/' crates/bevy_window/src/window.rs


add_category()
{
    category=$1
    category_path=`echo $category | tr '[:upper:]' '[:lower:]'`
    example_weight=0

    mkdir ../../content/examples/$category_path

    shift
    for example in $@
    do
        echo "building $category / $example"
        mkdir ../../content/examples/$category_path/$example
        cp -r assets ../../content/examples/$category_path/$example
        cp examples/$category_path/$example.rs ../../content/examples/$category_path/$example/
        cargo build --release --target wasm32-unknown-unknown --example $example
        wasm-bindgen --out-dir ../../content/examples/$category_path/$example --no-typescript --target web target/wasm32-unknown-unknown/release/examples/$example.wasm
        echo "+++
title = \"$example\"
template = \"example.html\"
weight = $example_weight

[extra]
header_message = \"Examples\"
+++" > ../../content/examples/$category_path/$example/index.md

        example_weight=$((example_weight+1))
    done

    echo "+++
title = \"$category\"
sort_by = \"weight\"
weight = $category_weight
+++" > ../../content/examples/$category_path/_index.md

    category_weight=$((category_weight+1))
}

mkdir ../../content/examples

echo "+++
title = \"Bevy Examples in WebGL2\"
template = \"examples.html\"
sort_by = \"weight\"

[extra]
header_message = \"Examples\"
+++" > ../../content/examples/_index.md

category_weight=0

add_category 2d rect sprite sprite_flipping sprite_sheet text2d
add_category 3d 3d_scene lighting load_gltf orthographic parenting pbr texture update_gltf_scene
add_category UI button text text_debug ui
add_category audio audio
add_category Game breakout alien_cake_addict
add_category Tools bevymark

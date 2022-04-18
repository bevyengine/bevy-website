#!/bin/sh

git init bevy
cd bevy
git remote add origin https://github.com/bevyengine/bevy
git pull --depth=1 origin latest

# remove markdown files from assets so that they don't get picked up by Zola
find assets -type f -name '*.md' -exec rm {} +

# setting a canvas by default to help with integration
sed -i.bak 's/canvas: None,/canvas: Some("#bevy".to_string()),/' crates/bevy_window/src/window.rs

# setting the asset folder root to the root url of this domain
sed -i.bak 's/asset_folder: "assets"/asset_folder: "\/assets\/examples\/"/' crates/bevy_asset/src/lib.rs


add_category()
{
    category=$1
    category_path=$2
    category_slug=`echo $category_path | tr '_' '-'`
    example_weight=0

    mkdir ../../content/examples/$category_slug

    # Remove first two arguments
    shift 2

    # Generate a markdown file for each example
    # These represent each example page
    for example in $@
    do
        echo "building $category / $example"
        example_slug=`echo $example | tr '_' '-'`
        code_filename="$example.rs"
        mkdir ../../content/examples/$category_slug/$example_slug
        cp examples/$category_path/$code_filename ../../content/examples/$category_slug/$example_slug/
        cargo build --release --target wasm32-unknown-unknown --example $example
        wasm-bindgen --out-dir ../../content/examples/$category_slug/$example_slug --no-typescript --target web target/wasm32-unknown-unknown/release/examples/$example.wasm
        echo "+++
title = \"$example\"
template = \"example.html\"
weight = $example_weight

[extra]
code_filename = \"$code_filename\"
code_path = \"examples/$category_path/$code_filename\"
header_message = \"Examples\"
+++" > ../../content/examples/$category_slug/$example_slug/index.md

        example_weight=$((example_weight+1))
    done

    # Generate category index
    echo "+++
title = \"$category\"
sort_by = \"weight\"
weight = $category_weight
+++" > ../../content/examples/$category_slug/_index.md

    category_weight=$((category_weight+1))
}

mkdir ../../content/examples
cp -r assets/ ../../static/assets/examples/

echo "+++
title = \"Bevy Examples in WebGL2\"
template = \"examples.html\"
sort_by = \"weight\"

[extra]
header_message = \"Examples\"
+++" > ../../content/examples/_index.md

category_weight=0

# Add categories
# - first param: the label that will show on the website
# - second param: `bevy/examples/???` folder name
# - rest params: space separated list of example files within the folder that want to be used
add_category 2D 2d rect sprite sprite_flipping sprite_sheet text2d mesh2d mesh2d_manual
add_category 3D 3d 3d_scene lighting load_gltf orthographic parenting pbr spherical_area_lights texture update_gltf_scene
add_category UI ui button text text_debug ui
add_category Audio audio audio
add_category Shader shader shader_instancing shader_material_glsl shader_material
add_category ECS ecs iter_combinations
add_category Games games breakout alien_cake_addict
add_category "Stress Tests" stress_tests bevymark

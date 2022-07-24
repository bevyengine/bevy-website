#!/bin/sh

# Print all executed lines
# set -x

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

export CARGO_TARGET_DIR="target"

if [ "$1" = --ci ] ; then
    # disable optimizations
    cargo_profile="dev"
    cargo_target_dir="debug"
    # Do not optimize on --ci check,
    # wasm-opt takes A LOT of time to run
    wasm_opt_flag="-O0"
else
    # enable optimizations
    cargo_profile="wasm-release"
    cargo_target_dir="wasm-release"
    # Optimize for size
    wasm_opt_flag="-Oz"
fi

examples_dir="../../content/examples"

add_category()
{
    category=$1
    category_path=$2
    category_slug=`echo $category_path | tr '_' '-'`
    example_weight=0

    mkdir "$examples_dir/$category_slug"

    # Remove first two arguments
    shift 2

    # Generate a markdown file for each example
    # These represent each example page
    for example in $@
    do
        echo "building $category / $example"
        example_slug=`echo $example | tr '_' '-'`
        code_filename="$example.rs"
        out_dir=$examples_dir/$category_slug/$example_slug
        mkdir $out_dir
        cp examples/$category_path/$code_filename $out_dir
        cargo build --profile $cargo_profile --target wasm32-unknown-unknown --example $example

        wasm-bindgen --out-dir $out_dir \
            --no-typescript --target web target/wasm32-unknown-unknown/$cargo_target_dir/examples/$example.wasm

        wasm-opt $wasm_opt_flag \
            --output "$out_dir/${example}_bg.wasm.optimized" \
            "$out_dir/${example}_bg.wasm"
        
        mv "$out_dir/${example}_bg.wasm.optimized" "$out_dir/${example}_bg.wasm"


        # Patch generated JS to allow to inject custom `fetch` with loading feedback.
        # See: https://github.com/bevyengine/bevy-website/pull/355
        sed -i.bak \
          -e 's/getObject(arg0).fetch(/window.bevyLoadingBarFetch(/' \
          -e 's/input = fetch(/input = window.bevyLoadingBarFetch(/' \
          "$out_dir/$example.js"

        echo "+++
title = \"$example\"
template = \"example.html\"
weight = $example_weight

[extra]
code_path = \"content/examples/$category_slug/$example_slug/$code_filename\"
github_code_path = \"examples/$category_path/$code_filename\"
header_message = \"Examples\"
+++" > $out_dir/index.md

        example_weight=$((example_weight+1))
    done

    # Generate category index
    echo "+++
title = \"$category\"
sort_by = \"weight\"
weight = $category_weight
+++" > $examples_dir/$category_slug/_index.md

    category_weight=$((category_weight+1))
}

mkdir $examples_dir
cp -r assets/ ../../static/assets/examples/

echo "+++
title = \"Bevy Examples in WebGL2\"
template = \"examples.html\"
sort_by = \"weight\"

[extra]
header_message = \"Examples\"
+++" > $examples_dir/_index.md

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

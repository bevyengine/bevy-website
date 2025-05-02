<!-- glTF labels: add enum to avoid misspelling and keep up-to-date list documented -->
<!-- https://github.com/bevyengine/bevy/pull/13586 -->

If you've been using [`glTF`] files for your scenes or looked at an example that does you've might have seen the _labels_ at the end of the asset path:

```rust
let model_pine = asset_server.load("models/trees/pine.gltf#Scene0");
let model_hen = asset_server.load("models/animals/hen.gltf#Scene0");
let animation_hen = asset_server.load("models/animals/hen.gltf#Aniamtion1"); // Oh no!
```

Notice the `#Scene0` syntax at the end. The glTF format is able to contain many things in a single file, including several scenes, animations, lights, and more.

These labels are a way of telling Bevy which part of the file we're loading.

However this is prone to user-error, and it looks like an error snuck in! The hen animation got the label `Aniamtion1` instead of `Animation1`.

No more! The above can now be re-written like so:

```rust
let hen = "models/animals/hen.gltf"; // Can re-use this more easily too
let model_pine = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/trees/pine.gltf"));
let model_hen = asset_server.load(GltfAssetLabel::Scene(0).from_asset(hen));
let animation_hen = asset_server.load(GltfAssetLabel::Animation(0).from_asset(hen)); // No typo!
```

Check out [`glTF label docs`] to know which parts you can query for.

[`glTF`]: https://www.khronos.org/gltf/
[`glTF label docs`]: https://docs.rs/bevy/0.14/bevy/gltf/enum.GltfAssetLabel.html

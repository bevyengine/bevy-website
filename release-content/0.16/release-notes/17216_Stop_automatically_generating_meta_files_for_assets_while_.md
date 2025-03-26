<!-- Stop automatically generating meta files for assets while using asset processing. -->
<!-- https://github.com/bevyengine/bevy/pull/17216 -->

In Bevy 0.12, we introduced "Assets V2". This was a complete rewrite of our asset system and
included all sorts of cool features. One of those was "asset preprocessing". This allows defining
a `Process` to apply to assets before they are loaded for use in the engine. This however
necessitated "meta files" for every asset in your project - meaning when you started using asset
preprocessing *at all*, your entire `assets` folder would be filled with these meta files
automatically (even for assets that don't need any preprocessing).

To alleviate this pain, enabling asset preprocessing no longer automatically writes meta files! This
makes it easier to enable asset preprocessing and adopt it gradually.

In addition, we've added `AssetServer::write_default_loader_meta_file_for_path` and
`AssetProcessor::write_default_meta_file_for_path` to allow users to explicitly generate the default
meta files for assets when necessary.

Consider enabling asset processing with:

```rust
app.add_plugins(DefaultPlugins.set(
    AssetPlugin {
        mode: AssetMode::Processed,
        ..default()
    }
));
```

Enabling the `bevy/asset_processor` feature will then process files automatically for you. See
[the asset processing example](https://github.com/bevyengine/bevy/blob/main/examples/asset/processing/asset_processing.rs)
for more details!

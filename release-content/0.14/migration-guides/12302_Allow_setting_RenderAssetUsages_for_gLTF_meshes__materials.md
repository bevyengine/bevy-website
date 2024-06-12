It is now possible configure whether meshes and materials should be loaded in the main world, the render world, or both with `GltfLoaderSettings`. The `load_meshes` field has been changed from a `bool` to a `RenderAssetUsages` bitflag, and a new `load_materials` field as been added.

You may need to update any gLTF `.meta` files:

<!-- This is technically RON, but it follows a syntax similar to Rust so we use that instead for syntax highlighting. -->

```rust
// Before
load_meshes: true

// After
load_meshes: ("MAIN_WORLD | RENDER_WORLD")
```

If you use `AssetServer::load_with_settings` instead when loading gLTF files, you will also have to update:

```rust
// Before
asset_server.load_with_settings("model.gltf", |s: &mut GltfLoaderSettings| {
    s.load_meshes = true;
});

// After
asset_server.load_with_settings("model.gltf", |s: &mut GltfLoaderSettings| {
    s.load_meshes = RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD;
});
```

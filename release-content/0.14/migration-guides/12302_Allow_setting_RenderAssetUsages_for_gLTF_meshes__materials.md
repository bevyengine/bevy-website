When loading gLTF assets with `asset_server.load_with_settings`, use `RenderAssetUsages` instead of `bool` when setting load_meshes e.g.

```rust
let _ = asset_server.load_with_settings("...", |s: &mut GltfLoaderSettings| {
    s.load_meshes = RenderAssetUsages::RENDER_WORLD;
});
```

Use the new load_materials field for controlling material load & retention behaviour instead of load_meshes.

gLTF .meta files need similar updates e.g

```rust
load_meshes: true,
```

to

```rust
load_meshes: ("MAIN_WORLD | RENDER_WORLD"),
```

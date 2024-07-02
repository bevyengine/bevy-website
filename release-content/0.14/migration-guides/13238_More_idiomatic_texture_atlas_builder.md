`TextureAtlasBuilder` has been modified to be more consistent with other builders. As part of this, most methods now return `&mut Self` instead of `Self` and `finish()` has been renamed to `build()`.

```rust
// 0.13
let (texture_atlas_layout, texture) = TextureAtlasBuilder::default()
    .padding(UVec2::default())
    .format(TextureFormat::bevy_default())
    .finish()
    .unwrap();

// 0.14
let (texture_atlas_layout, texture) = TextureAtlasBuilder::default()
    .padding(UVec2::default())
    .format(TextureFormat::bevy_default());
    .build() // This is now `build()`.
    .unwrap();
```

`RenderMaterials`, `RenderMaterials2d`, and `RenderUiMaterials` have all been replaced with the `RenderAssets` resource. If you need access a `PreparedMaterial<T>` using an `AssetId`, use `RenderAssets::get` instead.

Furthermore, the `RenderAsset` trait should now be implemented for destination types rather than source types. If you need to access the source type, use the `RenderAsset::SourceAsset` associated type.

```rust
// 0.13
impl RenderAsset for Image {
    type PreparedAsset = GpuImage;

    // ...
}

// 0.14
impl RenderAsset for GpuImage {
    type SourceAsset = Image;

    // ...
}
```

The `ImageScaleMode` component has been removed. Instead, `SpriteImageMode` and `NodeImageMode` have been created for a new field `image_mode` on both `Sprite` and `UiImage`

In most cases, this means code that spawns an entity with
```rust
(
    UiImage::new(image.clone()),
    ImageScaleMode::Sliced(slicer.clone()),
)
```
should be converted to:
```rust
(
    UiImage::new(image.clone())
        .with_mode(NodeImageMode::Sliced(slicer.clone())),
)
```

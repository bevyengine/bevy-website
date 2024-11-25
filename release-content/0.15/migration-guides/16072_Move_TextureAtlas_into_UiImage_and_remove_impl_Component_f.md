Before:

```rust
commands.spawn((
  UiImage::new(image),
  TextureAtlas { index, layout },
));
```

After:

```rust
commands.spawn(UiImage::from_atlas_image(image, TextureAtlas { index, layout }));
```

Before:

```rust
commands.spawn(UiImage {
    texture: some_image,
    ..default()
})
```

After:

```rust
commands.spawn(UiImage {
    image: some_image,
    ..default()
})
```

`SpriteSheetBundle` has been deprecated. Insert the `TextureAtlas` component alongside a `SpriteBundle` instead.

```rust
// 0.13
commands.spawn(SpriteSheetBundle {
    texture,
    atlas: TextureAtlas {
        layout,
        ..default()
    },
    ..default()
});
// 0.14
commands.spawn((
    SpriteBundle {
        texture,
        ..default()
    },
    TextureAtlas {
        layout,
        ..default()
    },
));
```

`AtlasImageBundle` has been deprecated. Insert the `TextureAtlas` component alongside an `ImageBundle` instead.

```rust
// 0.13
commands.spawn(AtlasImageBundle {
    image,
    atlas: TextureAtlas {
        layout,
        ..default()
    },
    ..default()
});
// 0.14
commands.spawn((
    ImageBundle {
        image,
        ..default()
    },
    TextureAtlas {
        layout,
        ..default()
    },
));
```

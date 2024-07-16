`SpriteSheetBundle` has been deprecated as part of a style and maintenance-motivated move towards optional components that add functionality, rather than a proliferation of bundles. Insert the `TextureAtlas` component alongside a `SpriteBundle` instead.

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

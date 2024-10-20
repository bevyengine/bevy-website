`TextureAtlasBuilder` no longer stores a mapping back to the original images in `TextureAtlasLayout`; that functionality has been added to a new struct, `TextureAtlasSources`, instead. This also means that the signature for `TextureAtlasBuilder::finish` has changed, meaning that calls of the form:

```rust
let (atlas_layout, image) = builder.build()?;
```

Will now change to the form:

```rust
let (atlas_layout, atlas_sources, image) = builder.build()?;
```

And instead of performing a reverse-lookup from the layout, like so:

```rust
let atlas_layout_handle = texture_atlases.add(atlas_layout.clone());
let index = atlas_layout.get_texture_index(&my_handle);
let handle = TextureAtlas {
    layout: atlas_layout_handle,
    index,
};
```

You can perform the lookup from the sources instead:

```rust
let atlas_layout = texture_atlases.add(atlas_layout);
let index = atlas_sources.get_texture_index(&my_handle);
let handle = TextureAtlas {
    layout: atlas_layout,
    index,
};
```

Additionally, `TextureAtlasSources` also has a convenience method, `handle`, which directly combines the index and an existing `TextureAtlasLayout` handle into a new `TextureAtlas`:

```rust
let atlas_layout = texture_atlases.add(atlas_layout);
let handle = atlas_sources.handle(atlas_layout, &my_handle);
```

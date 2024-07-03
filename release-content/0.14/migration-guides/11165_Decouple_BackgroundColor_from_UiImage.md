The [`BackgroundColor`] component now renders a solid-color background behind [`UiImage`] instead of tinting its color. Use the `color` field of `UiImage` for tinting.

```rust
// 0.13
ButtonBundle {
    image: UiImage::new(my_texture),
    background_color: my_color_tint.into(),
    ..default()
}

// 0.14
ButtonBundle {
    image: UiImage::new(my_texture).with_color(my_color_tint),
    ..default()
}
```

Some UI systems have been split or renamed.

- `bevy_ui::RenderUiSystem::ExtractNode` has been split into `ExtractBackgrounds`, `ExtractImages`, `ExtractBorders`, and `ExtractText`.
- `bevy_ui::extract_uinodes` has been split into `extract_uinode_background_colors` and `extract_uinode_images`.
- `bevy_ui::extract_text_uinodes` has been renamed to `extract_uinode_text`.

[`BackgroundColor`]: https://docs.rs/bevy/0.14.0/bevy/prelude/struct.BackgroundColor.html
[`UiImage`]: https://docs.rs/bevy/0.14.0/bevy/prelude/struct.UiImage.html#structfield.color

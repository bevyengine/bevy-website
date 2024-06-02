The [`BackgroundColor`] component now renders a solid-color background behind [`UiImage`] instead of tinting its color. Use the `color` field of `UiImage` for tinting.

```rust
// 0.13
ButtonBundle {
    background_color: my_color.into(),
    ..default()
}
// 0.14
ButtonBundle {
    image: UiImage::default().with_color(my_color),
    ..default()
}
```

```rust
// 0.13
fn button_system(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *color = my_color.into();
            }
            // ...
        }
    }
}
// 0.14
fn button_system(
    mut query: Query<(&Interaction, &mut UiImage), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut image) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                image.color = my_color;
            }
            // ...
        }
    }
}
```

Some UI systems have been split or renamed.

- `bevy_ui::RenderUiSystem::ExtractNode` has been split into `ExtractBackgrounds`, `ExtractImages`, `ExtractBorders`, and `ExtractText`.
- `bevy_ui::extract_uinodes` has been split into `bevy_ui::extract_uinode_background_colors` and `bevy_ui::extract_uinode_images`.
- `bevy_ui::extract_text_uinodes` has been renamed to `extract_uinode_text`.

[`BackgroundColor`]: https://docs.rs/bevy/0.14.0/bevy/prelude/struct.BackgroundColor.html
[`UiImage`]: https://docs.rs/bevy/0.14.0/bevy/prelude/struct.UiImage.html#structfield.color

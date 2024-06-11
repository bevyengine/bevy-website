UI images can now be given solid background colors:

![UI image with background color](ui_image_background_color.png)

The `BackgroundColor` component now works for UI images instead of applying a color tint on the image itself. You can still apply a color tint by setting `UiImage::color`. For example:

```rust
commands.spawn((
    ImageBundle {
        image: UiImage::new(texture_handle).with_color(image_color_tint),
        ..default()
    },
    BackgroundColor(ANTIQUE_WHITE.into()),
    Outline::new(Val::Px(8.0), Val::ZERO, CRIMSON.into()),
));
```

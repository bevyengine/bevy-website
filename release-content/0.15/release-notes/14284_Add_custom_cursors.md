<!-- Add custom cursors -->
<!-- https://github.com/bevyengine/bevy/pull/14284 -->

Previously Bevy's native window cursors supported only a fixed set of built-in OS cursors. Bevy now also supports arbitrary images as "custom cursors". Custom cursors still use native facilities of the OS, which allows them to stay perfectly responsive even when the frame rate of the application drops.

Insert the [`CursorIcon`] component with a [`CustomCursor`] to set a [`Window`] entity's cursor:

```rust
commands
    .entity(window)
    .insert(CursorIcon::Custom(CustomCursor::Image {
        handle: asset_server.load("cursor_icon.png"),
        hotspot: (5, 5),
    }));
```

[`CursorIcon`]: https://dev-docs.bevyengine.org/bevy/winit/cursor/enum.CursorIcon.html
[`CustomCursor`]: https://dev-docs.bevyengine.org/bevy/winit/cursor/enum.CustomCursor.html
[`Window`]: https://dev-docs.bevyengine.org/bevy/prelude/struct.Window.html
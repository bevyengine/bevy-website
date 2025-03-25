The `CustomCursor::Image` enum variant has some new fields. Update your code to set them.

Before:

```rust
CustomCursor::Image {
    handle: asset_server.load("branding/icon.png"),
    hotspot: (128, 128),
}
```

After:

```rust
CustomCursor::Image {
    handle: asset_server.load("branding/icon.png"),
    texture_atlas: None,
    flip_x: false,
    flip_y: false,
    rect: None,
    hotspot: (128, 128),
}
```

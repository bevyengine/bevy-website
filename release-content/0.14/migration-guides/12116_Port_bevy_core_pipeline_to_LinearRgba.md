`ColorAttachment::new()` now takes `Option<LinearRgba>` instead of `Option<Color>` for the `clear_color`. You can use the `From<Color>` implementation to convert your color.

```rust
let clear_color: Option<LinearRgba> = Some(color.into());
```

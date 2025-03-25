`RenderTarget::Image` now takes an `ImageRenderTarget` instead of a `Handle<Image>`. You can call `handle.into()` to construct an `ImageRenderTarget` using the same settings as before.

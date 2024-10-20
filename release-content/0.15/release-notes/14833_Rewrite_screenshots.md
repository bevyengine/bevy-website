Screenshots can now be taken with a new observer based API that allows targeting any `RenderTarget` that can be used with a `Camera`, not just windows. 

```rust
// Capture the primary window
commands
    .spawn(Screenshot::primary_window())
    .observe(save_to_disk(path));

// Or a `Handle<Image>`
commands
    .spawn(Screenshot::image(render_target))
    .observe(save_to_disk(path));
```

The observer triggers with a ScreenshotCaptured event containing an Image that can be used for saving to disk, post-processing, or generating thumbnails. This flexible approach makes it easier to capture content from any part of your rendering pipeline, whether it’s a window, an off-screen render target, or a texture in a custom render pass.

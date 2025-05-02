<!-- Throttle: https://github.com/bevyengine/bevy/pull/12622 -->

Using a lot of assets? Uploading lots of bytes to the GPU in a short time might cause stutters due to the render world waiting for uploads to finish.

Often it's a more delightful experience if an application runs smoothly than if it stutters, and a few frames worth of delay before seeing an asset appear
is often not even perceptible.

This experience is now possible:

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(RenderAssetBytesPerFrame::new(1_000_000_000)) // Tune to your situation by experimenting!
        .run();
}
```

That's it!
The number provided should be chosen by figuring out a nice trade-off between no stuttering and an acceptable delay.

This feature relies on assets knowing how many bytes they will occupy when sent to the GPU.
Currently this is known by images and meshes, with more assets types expected to be able to report this in the future.

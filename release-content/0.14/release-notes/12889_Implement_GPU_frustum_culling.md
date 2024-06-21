<!-- GPU Frustum Culling: https://github.com/bevyengine/bevy/pull/12889-->

Bevy's rendering stack is often CPU-bound: by shifting more work onto the GPU, we can better balance the load and render more shiny things faster.
Frustum culling is an optimization technique that automatically hides objects that are outside of a camera's view (its frustum).
In Bevy 0.14, users can choose to have this work performed on the GPU, depending on the performance characteristics of their project.

To enable it, add the `GpuCulling` component to the camera.

```rust
commands.spawn((
    Camera3dBundle::default(),
    GpuCulling
));
```

To disable CPU frustum culling, you'll need to add the `NoCpuCulling`
component to the camera.

```rust
commands.spawn((
    Camera3dBundle::default(),
    NoCpuCulling
));
```

**Note:** `GpuCulling` does not imply `NoCpuCulling`. To only perform frustum
culling on the GPU, you need to add both `GpuCulling` and `NoCpuCulling` to the
camera.

```rust
commands.spawn((
    Camera3dBundle::default(),
    GpuCulling,
    NoCpuCulling
));
```

<!-- GPU Frustum Culling: https://github.com/bevyengine/bevy/pull/12889-->

Frustum culling can now optionally be performed on the GPU.

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

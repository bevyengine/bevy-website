<!-- GPU Frustum Culling: https://github.com/bevyengine/bevy/pull/12889-->

Bevy's rendering stack is often CPU-bound: by shifting more work onto the GPU, we can better balance the load and render more shiny things faster.
Frustum culling is an optimization technique that automatically hides objects that are outside of a camera's view (its frustum).
In Bevy 0.14, users can choose to have this work performed on the GPU, depending on the performance characteristics of their project.

Two new components are available to control frustum culling: `GpuCulling` and `NoCpuCulling`. Attach the appropriate combination of these
components to a camera, and you're set.

```rust
commands.spawn((
    Camera3dBundle::default(),
    // Enable GPU frustum culling (does not automatically disable CPU frustum culling).
    GpuCulling,
    // Disable CPU frustum culling.
    NoCpuCulling
));
```

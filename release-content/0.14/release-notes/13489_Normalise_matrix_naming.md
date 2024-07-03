<!-- Normalize matrix naming -->
<!-- https://github.com/bevyengine/bevy/pull/13489 -->

Game engines generally provide a set of matrices to perform space transformations in the game world. Commonly, the following spaces are used:

- **Normalized Device Coordinates**: used by the graphics API directly
- **Clip Space**: coordinates after projection but before perspective divide
- **View Space**: coordinates in the camera's view
- **World Space**: global coordinates (this is the one we most often talk about!)
- **Model Space**: (or local space) coordinates relative to an entity

A common example is the 'model view projection matrix', which is the transformation from model space to NDC space (peculiarly in this shorthand,
the view matrix is often a transformation from world _to view_ space, but the model matrix is a transformation _from model_ (or local) space to world space).
Usually, matrices are referred to as part of that shorthand, so for example, the projection matrix transforms from view coordinates to NDC coordinates.

In a couple of places, Bevy had a view matrix, which was the transformation from view to world space (rather than from world to view space as above).
Additionally, even when used consistently, the single-word shorthands are ambiguous and can cause confusion. We felt that a clearer convention was needed.

From now on, matrices in Bevy are named `y_from_x`, for example `world_from_local`, which would denote the transformation from local to world-space coordinates.
One tidy benefit of this is that the inverse matrices are named `x_from_y`, and when multiplying between spaces, it's easy to see that it's correct.

For example, instead of writing:

```rust
let model_view_projection = projection * view * model;
```

You would now write:

```rust
let clip_from_local = clip_from_view * view_from_world * world_from_local;
```

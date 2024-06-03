<!-- Normalise matrix naming -->
<!-- https://github.com/bevyengine/bevy/pull/13489 -->

Most game engines use a matrix stack to represent the space transformations in the game world. The stack usually contains transformations for the following spaces:
- Normalized Device Coordinates: used by the graphics API directly
- View Space: coordinates in the camera's view
- World Space: global coordinates (this is the one we most often talk about!)
- Model Space: (or local space) coordinates relative to an entity

A common example of how these are named might be the 'model view projection matrix', which is the conversion from model space, to NDC space (peculiarly in this shorthand,
the view matrix is often a transformation from world to view space, but the model matrix is a transformation from model (or local) space to world space).
Usually, matrices in the stack are referred to by their part of that transformation, so for example, the projection matrix transforms from view coordinates to NDC coordinates.

In a couple of places, bevy had a view matrix, which was the transformation from view to world space. Additionally, even when used consistently, the single-word shorthands
are ambiguous and can cause confusion. We felt that a clearer convention was needed.

From now on, matrices in bevy are named `y_from_x`, for example `world_from_local`, which would denote the transformation from local to world-space coordinates.
One tidy benefit of this, is that the inverse matrices are named `x_from_y`, and when multiplying between spaces, it's easy to see that it's correct.

For example, instead of writing:
```rust
let model_view_projection = projection * view * model;
```
You might now write:
```rust
let clip_from_local = clip_from_view * view_from_world * world_from_local;
```

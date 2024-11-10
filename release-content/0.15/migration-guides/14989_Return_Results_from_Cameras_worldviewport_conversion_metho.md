The following methods on `Camera` now return a `Result` instead of an `Option` so that they can provide more information about failures:

- `world_to_viewport`
- `world_to_viewport_with_depth`
- `viewport_to_world`
- `viewport_to_world_2d`

Call `.ok()` on the `Result` to turn it back into an `Option`, or handle the `Result` directly.

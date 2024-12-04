`ScalingMode` has been refactored for clarity, especially on how to zoom orthographic cameras and their projections:

- `ScalingMode::WindowSize` no longer stores a float, and acts as if its value was 1. Divide your camera’s scale by any previous value to achieve identical results.
- `ScalingMode::FixedVertical` and `FixedHorizontal` now use named fields.

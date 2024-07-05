`CameraOutputMode::Write` now stores a `ClearColorConfig` instead of a `LoadOp<Color>`. Use the following table to convert between the two enums:

|`LoadOp<Color>`|`ClearColorConfig`|
|-|-|
|`Clear(color)`|`Custom(color)`|
|`Load`|`None`|

`ClearColorConfig` has an additional variant, `Default`, which inherits the clear color from the `ClearColor` resource.

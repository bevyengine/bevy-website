`Transform::rotate_axis()` and `Transform::rotate_local_axis()` now require a `Dir3` instead of a `Vec3` because the axis is expected to be normalized. In general you can call `Dir3::new()` with a `Vec3`, which will automatically normalize it, though you must handle the `Result` in case the vector is invalid.

Note that most constants like `Vec3::X` have a corresponding `Dir3` variant, such as `Dir3::X`.

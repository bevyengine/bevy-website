The `Bounded2d` and `Bounded3d` traits now take `Isometry2d` and `Isometry3d` parameters (respectively) instead of separate translation and rotation arguments. Existing calls to `aabb_2d`, `bounding_circle`, `aabb_3d`, and `bounding_sphere` will have to be changed to use isometries instead. A straightforward conversion is to refactor just by calling `Isometry2d/3d::new`, as follows:

```rust
// Old:
let aabb = my_shape.aabb_2d(my_translation, my_rotation);

// New:
let aabb = my_shape.aabb_2d(Isometry2d::new(my_translation, my_rotation));
```

However, if the old translation and rotation are 3d translation/rotations originating from a `Transform` or `GlobalTransform`, then `to_isometry` may be used instead. For example:

```rust
// Old:
let bounding_sphere = my_shape.bounding_sphere(shape_transform.translation, shape_transform.rotation);

// New:
let bounding_sphere = my_shape.bounding_sphere(shape_transform.to_isometry());
```

This discussion also applies to the `from_point_cloud` construction method of `Aabb2d`/`BoundingCircle`/`Aabb3d`/`BoundingSphere`, which has similarly been altered to use isometries.

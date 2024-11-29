<!-- Basic isometry types -->
<!-- https://github.com/bevyengine/bevy/pull/14269 -->

Vectors and quaternions are commonly used in 3D to describe relative and absolute positions and orientations of objects.
However, when performing more complicated transformations, such as going from a global frame of reference to an object's local space and back,
or composing multiple translations and rotations together, they can get rather unwieldy and difficult to reason about.

The new [`Isometry2d`] and [`Isometry3d`] types introduced in **Bevy 0.15** are a simple yet powerful tool for efficiently describing
these kinds of transformations. An isometry represents a rotation followed by a translation, similar to a [`Transform`] with a scale of 1.

```rust
// Create an isometry from a translation and rotation.
let iso1 = Isometry3d::new(Vec3::new(2.0, 1.0, 3.0), Quat::from_rotation_z(FRAC_PI_2));

// Transform a point using the isometry.
let point = Vec3::new(4.0, 4.0, 4.0);
let result = iso1.transform_point(point); // or iso1 * point
assert_relative_eq!(result, Vec3::new(-2.0, 5.0, 7.0));

// Create another isometry.
let iso2 = Isometry3d::from_rotation(Quat::from_rotation_z(FRAC_PI_2));

// Compute the relative translation and rotation.
let relative_iso = iso1.inverse_mul(iso2); // or iso1.inverse() * iso2
```

Isometries are most useful in mathematical contexts where scaling is not desired, such as when describing relative positions of objects
for intersection tests and other geometric queries. However, they are now also used in some APIs, including gizmo methods:

```rust
// Specify rectangle position and orientation with an isometry.
gizmos.rect_2d(Isometry2d::new(translation, Rot2::degrees(45.0)), Vec2::splat(250.0), CYAN);

// Many methods take an `impl Into<Isometry3d>`, so it is enough to only provide
// translation or rotation if a full isometry isn't needed.
gizmos.sphere(translation, 1.0, PURPLE);
```

[`Transform`] and [`GlobalTransform`] can also be converted to an [`Isometry3d`] using the [`to_isometry`] method,
providing a convenient way to use these APIs when you already have access to entity transforms.

Note that unlike [`Transform`], these isometry types are *not* components. They are purely convenience types for math.

[`Isometry2d`]: https://docs.rs/bevy/0.15/bevy/math/struct.Isometry2d.html
[`Isometry3d`]: https://docs.rs/bevy/0.15/bevy/math/struct.Isometry3d.html
[`Transform`]: https://docs.rs/bevy/0.15/bevy/transform/components/struct.Transform.html
[`GlobalTransform`]: https://docs.rs/bevy/0.15/bevy/transform/components/struct.GlobalTransform.html
[`to_isometry`]: https://docs.rs/bevy/0.15/bevy/transform/components/struct.Transform.html#method.to_isometry

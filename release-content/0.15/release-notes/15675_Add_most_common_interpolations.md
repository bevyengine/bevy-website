<!-- Add most common interpolations -->
<!-- https://github.com/bevyengine/bevy/pull/15675 -->

"Easing functions" can be used to easily construct curves that interpolate between two values.
There are many "common" easing functions that each have a different "character" to them. These
are often used in "tweening" scenarios to give life to the interpolation.

<video controls><source src="ease-functions.mp4" type="video/mp4"/></video>

**Bevy 0.15** adds a new `Ease` trait, which defines how to interpolate a value of a given type. The `Ease` types include:

* vector types (`f32`, `Vec2`, `Vec3`, ...);
* direction types (`Dir2`, `Dir3`, `Dir3A`);
* rotation types (`Rot2`, `Quat`).

We've also added an `EaseFunction` enum, which defines many common easing functions. The new `easing_curve` constructor uses these as inputs to define a final `Curve` from the given easing parameters.

For example, we can use an easing function to interpolate between two rotations:

```rust
// Ease between no rotation and a rotation of angle PI/2 about the y-axis.
let rotation_curve = easing_curve(
    Quat::IDENTITY,
    Quat::from_rotation_y(FRAC_PI_2),
    EaseFunction::ElasticInOut,
)
.reparametrize_linear(interval(0.0, 4.0).unwrap())
.unwrap();
```

<video controls><source src="eased-rotation.mp4" type="video/mp4"/></video>

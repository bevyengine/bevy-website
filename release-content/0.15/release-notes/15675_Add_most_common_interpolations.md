<!-- Add most common interpolations -->
<!-- https://github.com/bevyengine/bevy/pull/15675 -->

Many common easing functions can be used to easily construct curves that 
interpolate between two values; these may be used for things like building
one-shot animations, although they do not yet comprise a full tweening solution.

<video controls><source src="ease-functions.mp4" type="video/mp4"/></video>

For types that implement the `Ease` trait, the above easing functions can be
combined with start and end values with the `easing_curve` constructor to build
a curve that eases between the two in the given way.

These types include:
- vector types (`f32`, `Vec2`, `Vec3`, ...)
- direction types (`Dir2`, `Dir3`, `Dir3A`)
- rotation types (`Rot2`, `Quat`)

So, for example, we can use an easing function to interpolate between two 
rotations:
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

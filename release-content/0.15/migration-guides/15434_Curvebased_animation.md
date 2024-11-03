Most user code that does not directly deal with `AnimationClip` and `VariableCurve` will not need to be changed. On the other hand, `VariableCurve` has been completely overhauled. If you were previously defining animation curves in code using keyframes, you will need to migrate that code to use curve constructors instead. For example, a rotation animation defined using keyframes and added to an animation clip like this:

```rust
animation_clip.add_curve_to_target(
    animation_target_id,
    VariableCurve {
        keyframe_timestamps: vec![0.0, 1.0, 2.0, 3.0, 4.0],
        keyframes: Keyframes::Rotation(vec![
            Quat::IDENTITY,
            Quat::from_axis_angle(Vec3::Y, PI / 2.),
            Quat::from_axis_angle(Vec3::Y, PI / 2. * 2.),
            Quat::from_axis_angle(Vec3::Y, PI / 2. * 3.),
            Quat::IDENTITY,
        ]),
        interpolation: Interpolation::Linear,
    },
);
```

would now be added like this:

```rust
animation_clip.add_curve_to_target(
    animation_target_id,
    AnimatableKeyframeCurve::new([0.0, 1.0, 2.0, 3.0, 4.0].into_iter().zip([
        Quat::IDENTITY,
        Quat::from_axis_angle(Vec3::Y, PI / 2.),
        Quat::from_axis_angle(Vec3::Y, PI / 2. * 2.),
        Quat::from_axis_angle(Vec3::Y, PI / 2. * 3.),
        Quat::IDENTITY,
    ]))
    .map(RotationCurve)
    .expect("Failed to build rotation curve"),
);
```

Note that the interface of `AnimationClip::add_curve_to_target` has also changed (as this example shows, if subtly), and now takes its curve input as an `impl AnimationCurve`. If you need to add a `VariableCurve` directly, a new method `add_variable_curve_to_target` accommodates that (and serves as a one-to-one migration in this regard).

__For reviewers__

The diff is pretty big, and the structure of some of the changes might not be super-obvious:

- `keyframes.rs` became `animation_curves.rs`, and `AnimationCurve` is based heavily on `Keyframes`, with the adaptors also largely following suite.
- The Curve API adaptor structs were moved from `bevy_math::curve::mod` into their own module `adaptors`. There are no functional changes to how these adaptors work; this is just to make room for the specialized reflection implementations since `mod.rs` was getting kind of cramped.
- The new module `gltf_curves` holds the additional curve constructions that are needed by the glTF loader. Note that the loader uses a mix of these and off-the-shelf `bevy_math` curve stuff.
- `animatable.rs` no longer holds logic related to keyframe interpolation, which is now delegated to the existing abstractions in `bevy_math::curve::cores`.

<!-- Allow animation clips to animate arbitrary properties. -->
<!-- https://github.com/bevyengine/bevy/pull/15282 -->
[`AnimationClip`] can now be used to animate component fields with arbitrary curves.

```rust
animation_clip.add_curve_to_target(
    animation_target_id,
    AnimatableCurve::new(
        animated_field!(TextFont::font_size),
        // Oscillate the font size during the length of the animation.
        FunctionCurve::new(
            Interval::UNIT, 
            |t| 25.0 * f32::sin(TAU * t) + 50.0
        )
    )
);
```

<video controls><source src="animated-font-size.mp4" type="video/mp4"/></video>

This works for any named field and uses the new `Curve` API, which supports arbitrary curve types.
Animating `Transform` fields will likely be the most common use case:

```rust
animation_clip.add_curve_to_target(
    animation_target_id,
    AnimatableCurve::new(
        animated_field!(Transform::translation),
        // Construct a `Curve<Vec3>`` using a built-in easing curve constructor.
        EasingCurve::new(
            vec3(-10., 2., 0.),
            vec3(6., 2., 0.),
            EaseFunction::CubicInOut,
        )
    )
);
```

Bevy's internal animation handling for things like GLTF animations uses the same API!

If you need more complicated logic than "animate a specific component field", you can implement [`AnimatableProperty`], which can be used in
[`AnimatableCurve`] in place of [`animated_field!`].

[`AnimationClip`]: https://dev-docs.bevyengine.org/bevy/animation/struct.AnimationClip.html
[`AnimatableProperty`]: https://dev-docs.bevyengine.org/bevy/animation/animation_curves/trait.AnimatableProperty.html
[`AnimatableCurve`]: https://dev-docs.bevyengine.org/bevy/animation/animation_curves/struct.AnimatableCurve.html
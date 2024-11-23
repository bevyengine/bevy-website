<!-- Allow animation clips to animate arbitrary properties. -->
<!-- https://github.com/bevyengine/bevy/pull/15282 -->
Animation clips can now be used to animate component properties with arbitrary curves.

```rust
animation_clip.add_curve_to_target(
    animation_target_id,
    AnimatableCurve::new(
        AnimatedProperty::new(|font: &mut TextFont| &mut font.font_size),
        // Oscillate the font size during the length of the animation.
        FunctionCurve::new(
            Interval::UNIT, 
            |t| 25.0 * f32::sin(TAU * t) + 50.0
        )
    )
);
```

This uses the new `Curve` API, which supports arbitrary curve types.

<video controls><source src="animated-font-size.mp4" type="video/mp4"/></video>

Similarly, animations of `Transform` components and morph weights may also be
specified using arbitrary curves; for example, the type `TranslationCurve` wraps
a `Curve<Vec3>` and uses it to animate the `translation` part of the target's
`Transform`.

```rust
// Construct a `Curve<Vec3>`` using a built-in easing curve constructor.
let translation_curve = EasingCurve::new(
    vec3(-10., 2., 0.),
    vec3(6., 2., 0.),
    EaseFunction::CubicInOut,
);

// Use this `Curve<Vec3>` to animate the `translation` part of `Transform`.
animation_clip.add_curve_to_target(
    animation_target_id,
    TranslationCurve(translation_curve)
);
```

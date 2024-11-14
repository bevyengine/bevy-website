<!-- Allow animation clips to animate arbitrary properties. -->
<!-- https://github.com/bevyengine/bevy/pull/15282 -->
Animation clips can be used to animate arbitrary component properties with arbitrary curves.
This is driven by an `AnimatableProperty` trait, which reaches into a component
and selects part of it to modify.

For example:
```rust
// A marker struct that we can use to target font size with animations.
// This pattern is typical.
#[derive(Reflect)]
struct FontSizeProperty;

impl AnimatableProperty for FontSizeProperty {
    // The `TextFont` component is what will actually be modified by animation.
    type Component = TextFont;

    // In order to drive the animation of the font size, we will need something
    // that outputs an `f32`.
    type Property = f32;

    // This reaches into the `TextFont` component and grabs the font size.
    fn get_mut(component: &mut Self::Component) -> Option<&mut Self::Property> {
        Some(&mut component.font_size)
    }
}
```

In concert with the new `Curve` API, this allows any `Curve<f32>` to be used to
animate the font size of an entity:
```rust
// Create a new animation clip to hold our animation.
let mut animation_clip = AnimationClip::default();

// Oscillate the font size during the length of the animation.
let oscillating_curve = FunctionCurve::new(
    Interval::UNIT, 
    |t| 25.0 * f32::sin(TAU * t) + 50.0
);

// The curve itself is a `Curve<f32>`, and the usage of `FontSizeProperty` tells
// the animation system how to use that curve to actually animate something.
animation_clip.add_curve_to_target(
    animation_target_id,
    AnimatableCurve::<FontSizeProperty, _>::from_curve(oscillating_curve)
);
```

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

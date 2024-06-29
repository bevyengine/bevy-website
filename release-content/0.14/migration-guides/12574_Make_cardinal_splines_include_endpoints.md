There was a bug in `CubicCardinalSpline` where the curve would only pass through the interior control points, not the points at the beginning and end. (For an in-depth analysis, see [this issue](https://github.com/bevyengine/bevy/issues/12570).) This has been fixed so that the curve passes through all control points, but it may break behavior you were depending on.

If you rely on the old behavior of `CubicCardinalSpline`, you will have to truncate any parametrizations you used in order to access a curve identical to the one you had previously. This can be done by chopping off a unit-distance segment from each end of the parametrizing interval. For instance, if your code looks as follows:

```rust
fn interpolate(t: f32) -> Vec2 {
    let points = [
        vec2(-1.0, -20.0),
        vec2(3.0, 2.0),
        vec2(5.0, 3.0),
        vec2(9.0, 8.0),
    ];
    let my_curve = CubicCardinalSpline::new(0.3, points).to_curve();
    my_curve.position(t)
}
```

Then in order to obtain similar behavior, `t` will need to be shifted up by 1 (since the output of `CubicCardinalSpline::to_curve` has introduced a new segment in the interval [0,1]), displacing the old segment from [0,1] to [1,2]:

```rust
fn interpolate(t: f32) -> Vec2 {
    let points = [
        vec2(-1.0, -20.0),
        vec2(3.0, 2.0),
        vec2(5.0, 3.0),
        vec2(9.0, 8.0),
    ];
    let my_curve = CubicCardinalSpline::new(0.3, points).to_curve();
    // Add 1 here to restore original behavior.
    my_curve.position(t + 1)
}
```

(Note that this does not provide identical output for values of `t` outside of the interval [0,1].)

On the other hand, any user who was specifying additional endpoint tangents simply to get the curve to pass through the right points (i.e. not requiring exactly the same output) can simply omit the endpoints that were being supplied only for control purposes.

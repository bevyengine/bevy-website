Any users relying on the old behavior of `CubicCardinalSpline` will have to truncate any parametrizations they used in order to access a curve identical to the one they had previously. This would be done by chopping off a unit-distance segment from each end of the parametrizing interval. For instance, if a user’s existing code looks as follows

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

then in order to obtain similar behavior, `t` will need to be shifted up by 1, since the output of `CubicCardinalSpline::to_curve` has introduced a new segment in the interval [0,1], displacing the old segment from [0,1] to [1,2]:

```rust
fn interpolate(t: f32) -> Vec2 {
    let points = [
        vec2(-1.0, -20.0),
        vec2(3.0, 2.0),
        vec2(5.0, 3.0),
        vec2(9.0, 8.0),
    ];
    let my_curve = CubicCardinalSpline::new(0.3, points).to_curve();
    my_curve.position(t+1)
}
```

(Note that this does not provide identical output for values of `t` outside of the interval [0,1].)

On the other hand, any user who was specifying additional endpoint tangents simply to get the curve to pass through the right points (i.e. not requiring exactly the same output) can simply omit the endpoints that were being supplied only for control purposes.

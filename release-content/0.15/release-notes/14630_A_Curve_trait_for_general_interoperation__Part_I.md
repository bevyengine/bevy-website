<!-- A Curve trait for general interoperation — Part I -->
<!-- https://github.com/bevyengine/bevy/pull/14630 -->

The new [`Curve<T>`] trait provides a shared interface for curves, describing how values of type `T` change as we vary a `f32` parameter `t` over some domain.

What's changing, and the domain that it's changing *over* are both incredibly flexible.
You might choose to set `T` to store colors, creating a powerful abstraction for [color gradients](https://docs.rs/bevy/0.15.0/bevy/color/struct.ColorCurve.html), position, or the experience your player needs to level up.

As we vary the progress parameter `t`, we could be representing time (like for animation),
or something like distance or displacement as for curves that are mapped into 2D or 3D space,
or a fraction of progress between a starting and ending value.

## Constructing Curves

Each curve made be defined in a variety of ways. For example, a curve may be:

* defined by a function
* interpolated from samples
* constructed using splines
* produced by an easing function

Take a look at the constructors on the [`Curve<T>`] trait for more details.

## Modifying curves

Procedurally modifying curves is a powerful tool for both creating curves with the desired behavior and dynamically altering them.

Bevy 0.15 provides a number of flexible adaptors for taking an existing curve and
modifying its output and/or parametrization.

For example:

```rust
let timed_angles = [
  (0.0, 0.0),
  (1.0, -FRAC_PI_2),
  (2.0, 0.0),
  (3.0, FRAC_PI_2),
  (4.0, 0.0)
];

// A curve interpolating our list of (time, angle)-pairs. At each time, it
// produces the angle, so it is a `Curve<f32>` parametrized over `[0, 4]`.
let angle_curve = UnevenSampleAutoCurve::new(timed_angles).unwrap();

// Interpret these angles as angles of rotation for a `Curve<Rot2>`.
let rotation_curve = angle_curve.map(Rot2::radians);

// Change the parameterizing interval so that the whole loop happens in
// only 1 second instead of 4.
let fast_rotation_curve = rotation_curve.reparametrize_linear(Interval::UNIT).unwrap();
```

A number of other adaptors are also available. For instance:

* a curve may be reversed, repeated, or ping-ponged
* two curves may be chained together to form a longer curve
* two curves may be zipped together to form a curve valued in tuples

## Sampling from curves

Sampling is the process of asking "what is the value of this curve at some particular value of `t`".
Doing so is simple: just call [`Curve::sample`]!

Curves can also be rasterized into regular, discretized intervals.
By resampling into an approximation derived from sample interpolation
on the original curve, we can make curves of diverse origin
uniform at the level of data.

While this may seem exotic, this technique is critical for serializing curves or
approximating properties via numerical methods.

```rust
// A curve defined by a function, which may be challenging to store as data.
let exponential_curve = FunctionCurve::new(
  interval(0.0, 10.0).unwrap(), 
  |t| f32::exp(2.0 * t)
);

// A curve approximating the original by resampling on 100 segments.
// Internally, this just holds the samples and the parameter interval.
let raster_curve = exponential_curve.resample_auto(100).unwrap();
```

[`Curve<T>`]: https://docs.rs/bevy/0.15.0/bevy/math/trait.Curve.html
[`EasingCurve`]: https://docs.rs/bevy/0.15.0/bevy/math/curve/struct.EasingCurve.html
[`EaseFunction`]: https://docs.rs/bevy/0.15.0/bevy/math/curve/enum.EaseFunction.html
[`Curve::sample`]: https://docs.rs/bevy/0.15.0/bevy/math/trait.Curve.html#method.sample

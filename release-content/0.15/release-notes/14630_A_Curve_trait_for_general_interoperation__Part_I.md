<!-- A Curve trait for general interoperation — Part I -->
<!-- https://github.com/bevyengine/bevy/pull/14630 -->

The new `Curve` trait provides a shared interface for curves.

`Curve<T>` defines a value of some type `T`
parametrized by a nonempty closed interval of real numbers. That parameter could,
for example, represent time, in which case a `Curve<T>` is thought of as a value
of type `T` that changes over time, as in animation. The parameter
could also represent something like distance or displacement, as in gradients and 
spatial curves.

The curves themselves may be defined in a variety of ways. For example, a curve may be:

* defined by a function
* interpolated from samples
* constructed using splines
* produced by an easing function

Additionally, the `Curve` API provides adaptors for taking an existing curve and
modifying its output and/or parametrization. It is similar to the `Iterator` 
interface in this way.

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

The interface additionally provides facilities for rasterization. These allow
a curve to be resampled into an approximation derived from sample interpolation
on the original curve; in practice, this is useful when curves of diverse origin
need to be made uniform at the level of data - e.g. in serialization or when
applying numerical methods.

```rust
// A curve defined by a function, which may be challenging to store as data.
let exponential_curve = function_curve(
  interval(0.0, 10.0).unwrap(), 
  |t| f32::exp(2.0 * t)
);

// A curve approximating the original by resampling on 100 segments.
// Internally, this just holds the samples and the parameter interval.
let raster_curve = exponential_curve.resample_auto(100).unwrap();
```

`bevy_math` has collected a sizable collection of curves and methods for working with curves, which are useful for everything from animations to color gradients to gameplay logic.

One of the most natural and important things you might want to do with a curve is to inspect its **derivative**:
the rate at which it's changing.
You might even be after its **second derivative**: the rate at which the rate of change is changing.

Accessing this information should be easy and error-resistant,
but we also won't accept any complication or performance penalty in the more common derivative-less use cases.

To satisfy these requirements, our resident mathematicians have built a careful solution involving multiple traits and wrapper types.
For a full description of the mechanisms and considerations, dive down the rabbit hole into the linked PR description!
Despite (or because of?) all of the mathematical type wizardry, the actual usage is straightforward:

```rust
let points = [
    vec2(-1.0, -20.0),
    vec2(3.0, 2.0),
    vec2(5.0, 3.0),
    vec2(9.0, 8.0),
];

// A cubic spline curve that goes through `points`.
let curve = CubicCardinalSpline::new(0.3, points).to_curve().unwrap();

// Calling `with_derivative` causes derivative output to be included in the output of the curve API.
let curve_with_derivative = curve.with_derivative();

// A `Curve<f32>` that outputs the speed of the original.
let speed_curve = curve_with_derivative.map(|x| x.derivative.norm());
```

We've implemented the required traits for most of our native curve types: splines, lines, and all manner of compound curves.
Curves which accept arbitrary functions are not covered (build your own specialized curve types),
as Rust does not have a first-class notion of a differentiable function!

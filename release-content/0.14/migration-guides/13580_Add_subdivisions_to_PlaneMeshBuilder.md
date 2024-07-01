In 0.13 the `Plane` type was deprecated in favor of `Plane2d` and `Plane3d`. The new plane types did not provide a method for subdivision, which is now amended.

If you used the `Plane::subdivisions` property, you now need to convert a `Plane3d` into a `PlaneMeshBuilder`.

```rust
// 0.13
let plane = Plane {
    subdivisions: 10,
    ..default()
};

// 0.14
let plane = Plane3d::default().mesh().subdivisions(10);
```

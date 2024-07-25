The `Plane3d` primitive is now a finite plane with a `half_size` field. If you want an infinite plane, use the new `InfinitePlane3d`.

```rust
// 0.13
let plane = Plane3d::new(Vec3::Y);

// 0.14
let plane = Plane3d {
    normal: Dir3::Y,
    half_size: Vec2::new(10., 10.),
};
let plane = InfinitePlane3d::new(Vec3::Y);
```

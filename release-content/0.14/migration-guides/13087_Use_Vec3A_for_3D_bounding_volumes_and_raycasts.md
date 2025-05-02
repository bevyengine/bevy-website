`Aabb3d`, `BoundingSphere`, and `RayCast3d` now use `Vec3A` instead of `Vec3` internally. `Vec3A` is the SIMD-accelerated form of `Vec3`, so it should provide performance improvements without visible changes in behavior.

If you manually construct any of the affected structs, you will have to convert into a `Vec3A`.

```rust
// 0.13
let x = Vec3::new(5.0, -2.0);

let aabb = Aabb3d {
    min: Vec3::ZERO,
    max: x,
};

// 0.14
let x = Vec3::new(5.0, -2.0);

let aabb = Aabb3d {
    // Both variants are very similar, so you can usually replace `Vec3` with `Vec3A`.
    min: Vec3A::ZERO,
    // In cases where you cannot, use the `From` and `Into` traits.
    max: x.into(),
};
```

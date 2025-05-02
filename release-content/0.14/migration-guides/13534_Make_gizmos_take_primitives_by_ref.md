`Gizmos::primitive_2d()` and `Gizmos::primitive_3d()` now take the primitive as a reference so that non-`Copy` primitives do not need to be cloned each time they are drawn.

```rust
// 0.13
fn draw(mut gizmos: Gizmos) {
    let polygon = Polygon {
        vertices: [
            // ...
        ],
    };

    // Since `Polygon` is not `Copy`, you would need to clone it if you use it more than once.
    gizmos.primitive_2d(polygon.clone(), Vec2::ZERO, 0.0, Color::WHITE);
    gizmos.primitive_2d(polygon, Vec2::ONE, 0.0, Color::BLACK);
}

// 0.14
fn draw(mut gizmos: Gizmos) {
    let polygon = Polygon {
        vertices: [
            // ...
        ],
    };

    // No need to clone the polygon anymore!
    gizmos.primitive_2d(&polygon, Vec2::ZERO, 0.0, Color::WHITE);
    gizmos.primitive_2d(&polygon, Vec2::ONE, 0.0, Color::BLACK);
}
```

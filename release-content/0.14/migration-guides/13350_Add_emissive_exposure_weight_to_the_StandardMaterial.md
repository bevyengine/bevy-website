Emissive color and camera exposure now play nicely with each other. Before, the `emissive` property of a `StandardMaterial` had to be massive (in the thousands) in order for effects such as bloom to be visible. This has been scaled down, so you may have to re-adjust your emissive colors.

```rust
// 0.13
StandardMaterial {
    emissive: Color::linear_rgb(23000.0, 9000.0, 3000.0),
    ..default()
}

// 0.14
StandardMaterial {
    // Much more reasonable! :)
    emissive: Color::linear_rgb(13.99, 5.32, 2.0),
    ..default()
}
```

You may also be interested in the `StandardMaterial::emissive_exposure_weight` property.

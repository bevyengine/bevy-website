Multiplying and dividing a `LinearRgba` by an `f32` used to skip the alpha channel, but now it is modified.

```rust
// Before
let color = LinearRgba {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
    alpha: 1.0,
} * 0.5;

// Alpha is preserved, ignoring the multiplier.
assert_eq!(color.alpha, 1.0);

// After
let color = LinearRgba {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
    alpha: 1.0,
} * 0.5;

// Alpha is included in multiplication.
assert_eq!(color.alpha, 0.5);
```

If you need the alpha channel to remain within the valid range from 0.0 to 1.0, consider clamping it:

```rust
let mut color = LinearRgba {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
    alpha: 1.0,
} * 10.0;

color.alpha = color.alpha.clamp(0.0, 1.0);
```

Note that in some cases, such as rendering sprites, the alpha is automatically clamped so you do not need to do it manually.

If you need the alpha channel to remain constant, consider overwriting it:

```rust
let color = LinearRgba {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
    alpha: 1.0,
};

// Overwrite the alpha to always be 1.0.
let new_color = (color / 2.0).with_alpha(1.0);
```

Multiplying and dividing a `LinearRgba` by an `f32` used to skip the alpha channel, but now it is modified.

```rust
// Before
let color = Color::RgbaLinear {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
    alpha: 1.0,
} * 0.5;

// Alpha is preserved, ignoring the multiplier.
assert_eq!(color.a(), 1.0);

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

If you need the alpha channel to remain untouched, consider creating your own helper method:

```rust
fn legacy_div_f32(color: &mut LinearRgba, scale: f32) {
    color.red /= scale;
    color.green /= scale;
    color.blue /= scale;
}

let mut color = LinearRgba {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
    alpha: 1.0,
};

legacy_div_f32(&mut color, 2.0);
```

If you are fine with the alpha changing, but need it to remain within the range of 0.0 to 1.0, consider clamping it:

```rust
let mut color = LinearRgba {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
    alpha: 1.0,
} * 10.0;

color.alpha = color.alpha.clamp(0.0, 1.0);
```

<!-- TODO: I want this to be a callout, but shortcodes don't work here. -->
Note that in some cases, such as rendering sprites, the alpha is automatically clamped so you do not need to do it manually.

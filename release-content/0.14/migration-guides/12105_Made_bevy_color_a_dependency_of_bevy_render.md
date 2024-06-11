The `SrgbColorSpace` trait, `HslRepresentation` struct, and `LchRepresentation` struct have been removed in favor of the specific color space structs.

For `SrgbColorSpace`, use `Srgba::gamma_function()` and `Srgba::gamma_function_inverse()`. If you used the `SrgbColorSpace` implementation for `u8`, convert it to an `f32` first:

```rust
// 14 is random, this could be any number.
let nonlinear: u8 = 14;

// Apply gamma function, converting `u8` to `f32`.
let linear: f32 = Srgba::gamma_function(nonlinear as f32 / 255.0);

// Convert back to a `u8`.
let linear: u8 = (linear * 255.0) as u8;
```

Note that this conversion can be costly, especially if called during the `Update` schedule. Consider just using `f32` instead.

`HslRepresentation` and `LchRepresentation` can be replaced with the `From` implementations between `Srgba`, `Hsla`, and `Lcha`.

```rust
// Before
let srgb = HslRepresentation::hsl_to_nonlinear_srgb(330.0, 0.7, 0.8);
let lch = LchRepresentation::nonlinear_srgb_to_lch([0.94, 0.66, 0.8]);

// After
let srgba: Srgba = Hsla::new(330.0, 0.7, 0.8, 1.0).into();
let lcha: Lcha = Srgba::new(0.94, 0.66, 0.8, 1.0).into();
```

Bevy's color support has received a major overhaul, and with it the new `bevy::color` module. Buckle up, many things have been changed!

#### Color space representation

Bevy's main `Color` enum is used to represent color in many different color spaces (such as RGB, HSL, and more). Before, these color spaces were all represented inline as variants:

```rust
enum Color {
    Rgba {
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    },
    Hsla {
        hue: f32,
        saturation: f32,
        lightness: f32,
        alpha: f32,
    },
    // ...
}
```

This has been changed so now each color space has its own dedicated struct:

```rust
struct Srgba {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

struct Hsla {
    hue: f32,
    saturation: f32,
    lightness: f32,
    alpha: f32,
}

enum Color {
    Srgba(Srgba),
    Hsla(Hsla),
    // ...
}
```

This makes it easier to organize and manage different color spaces, and makes room to add more! To handle this change, you may need to update your match statements:

```rust
// Before
match color {
    Color::Rgba { red, green, blue, alpha } => {
        // Something cool here!
    },
    _ => {},
}

// After
match color {
    Color::Srgba(Srgba { red, green, blue, alpha }) => {
        // Something cool here!
    },
    _ => {}
}
```

Additionally, you must now use the `From` and `Into` implementations when converting between color spaces, as compared to the old helper methods such as `as_rgba` and `as_hsla`.

```rust
// Before
let color = Color::rgb(1.0, 0.0, 1.0).as_hsla();

// After
let color: Hsla = Srgba::rgb(1.0, 0.0, 1.0).into();
```

#### `Color` methods

Any mention of RGB has been renamed to [sRGB]. This includes the variant `Color::Rgba` turning into `Color::Srgba` as well as methods such as `Color::rgb` and `Color::rgb_u8` turning into `Color::srgb` and `Color::srgb_u8`.

[sRGB]: https://en.wikipedia.org/wiki/SRGB

Methods to access specific channels of `Color` have been removed due to causing silent, relatively expensive conversions. This includes `Color::r`, `Color::set_r`, `Color::with_r`, and all of the equivalents for `g`, `b` `h`, `s` and `l`. Convert your `Color` into the desired color space, perform your operation there, and then convert it back.

```rust
// Before
let mut color = Color::rgb(0.0, 0.0, 0.0);
color.set_b(1.0);

// After
let color = Color::srgb(0.0, 0.0, 0.0);
let srgba = Srgba {
    blue: 1.0,
    ..Srgba::from(color),
};
let color = Color::from(srgba);
```

`Color::hex` has been moved to `Srgba::hex`. Call `.into()` or construct a `Color::Srgba` variant manually to convert it.

`Color::rgb_linear` and `Color::rgba_linear` have been renamed `Color::linear_rgb` and `Color::linear_rgba` to fit the naming scheme of the `LinearRgba` struct.

`Color::as_linear_rgba_f32` and `Color::as_linear_rgba_u32` have been removed. Call `LinearRgba::to_f32_array` and `LinearRgba::to_u32` instead, converting if necessary.

Several other color conversion methods to transform LCH or HSL colors into float arrays or `Vec` types have been removed. Please reimplement these externally or open a PR to re-add them if you found them particularly useful.

Vector field arithmetic operations on `Color` (add, subtract, multiply and divide by a f32) have been removed. Instead, convert your colors into `LinearRgba` space and perform your operations explicitly there. This is particularly relevant when working with emissive or HDR colors, whose color channel values are routinely outside of the ordinary 0 to 1 range.

#### Alpha

Alpha, also known as transparency, used to be referred to by the letter `a`. It is now called by its full name within structs and methods.

- `Color::set_a`, `Color::with_a`, and `Color::a` are now `Color::set_alpha`, `Color::with_alpha`, and `Color::alpha`. These are part of the new `Alpha` trait.
- Additionally, `Color::is_fully_transparent` is now part of the `Alpha`.

#### CSS Constants

The various CSS color constants are no longer stored directly on `Color`. Instead, they’re defined in the `Srgba` color space, and accessed via `bevy::color::palettes`. Call `.into()` on them to convert them into a `Color` for quick debugging use.

```rust
// Before
let color = Color::BLUE;

// After
use bevy::color::palettes::css::BLUE;

let color = BLUE;
```

Please note that `palettes::css` is not necessarily 1:1 with the constants defined previously as some names and colors have been changed to conform with the CSS spec. If you need the same color as before, consult the table below or use the color values from the [old constants](https://github.com/bevyengine/bevy/blob/v0.13.2/crates/bevy_render/src/color/mod.rs#L60).

|0.13|0.14|
|-|-|
|`GREEN`|`LIMEGREEN`|
|`PINK`|`DEEP_PINK`|
|`DARK_GRAY`|`Srgba::gray(0.25)`|

#### Switch to `LinearRgba`

`WireframeMaterial`, `ExtractedUiNode`, `ExtractedDirectionalLight`, `ExtractedPointLight`, `ExtractedSpotLight`, and `ExtractedSprite` now store a `LinearRgba` rather than a polymorphic `Color`. Furthermore, `Color` no longer implements `AsBindGroup`. You should store a `LinearRgba` instead to avoid conversion costs.

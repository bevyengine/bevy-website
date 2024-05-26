Bevy’s color types have changed! Wherever you used a `bevy::render::Color`, a `bevy::color::Color` is used instead.

These are quite similar! Both are enums storing a color in a specific color space (or to be more precise, using a specific color model). However, each of the different color models now has its own type.

TODO…

- `Color::rgba`, `Color::rgb`, `Color::rbga_u8`, `Color::rgb_u8`, `Color::rgb_from_array` are now `Color::srgba`, `Color::srgb`, `Color::srgba_u8`, `Color::srgb_u8` and `Color::srgb_from_array`.
- `Color::set_a` and `Color::a` is now `Color::set_alpha` and `Color::alpha`. These are part of the `Alpha` trait in `bevy_color`.
- `Color::is_fully_transparent` is now part of the `Alpha` trait in `bevy_color`
- `Color::r`, `Color::set_r`, `Color::with_r` and the equivalents for `g`, `b` `h`, `s` and `l` have been removed due to causing silent relatively expensive conversions. Convert your `Color` into the desired color space, perform your operations there, and then convert it back into a polymorphic `Color` enum.
- `Color::hex` is now `Srgba::hex`. Call `.into` or construct a `Color::Srgba` variant manually to convert it. 
- `WireframeMaterial`, `ExtractedUiNode`, `ExtractedDirectionalLight`, `ExtractedPointLight`, `ExtractedSpotLight`  and `ExtractedSprite` now store a `LinearRgba`, rather than a polymorphic `Color`
- `Color::rgb_linear` and `Color::rgba_linear` are now `Color::linear_rgb` and `Color::linear_rgba`
- The various CSS color constants are no longer stored directly on `Color`. Instead, they’re defined in the `Srgba` color space, and accessed via `bevy::color::palettes::css`. Call `.into()` on them to convert them into a `Color` for quick debugging use, and consider using the much prettier `tailwind` palette for prototyping.
- The `LIME_GREEN` color has been renamed to `LIMEGREEN` to comply with the standard naming.
- Vector field arithmetic operations on `Color` (add, subtract, multiply and divide by a f32) have been removed. Instead, convert your colors into `LinearRgba` space, and perform your operations explicitly there. This is particularly relevant when working with emissive or HDR colors, whose color channel values are routinely outside of the ordinary 0 to 1 range.
- `Color::as_linear_rgba_f32` has been removed. Call `LinearRgba::to_f32_array` instead, converting if needed.
- `Color::as_linear_rgba_u32` has been removed. Call `LinearRgba::to_u32` instead, converting if needed.
- Several other color conversion methods to transform LCH or HSL colors into float arrays or `Vec` types have been removed. Please reimplement these externally or open a PR to re-add them if you found them particularly useful.
- Various methods on `Color` such as `rgb` or `hsl` to convert the color into a specific color space have been removed. Convert into `LinearRgba`, then to the color space of your choice.
- Various implicitly-converting color value methods on `Color` such as `r`, `g`, `b` or `h` have been removed. Please convert it into the color space of your choice, then check these properties.
- `Color` no longer implements `AsBindGroup`. Store a `LinearRgba` internally instead to avoid conversion costs.

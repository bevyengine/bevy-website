__`bevy_render::color::colorspace::SrgbColorSpace::<f32>::linear_to_nonlinear_srgb`__

Use `bevy_color::color::gamma_function_inverse`

__`bevy_render::color::colorspace::SrgbColorSpace::<f32>::nonlinear_to_linear_srgb`__

Use `bevy_color::color::gamma_function`

__`bevy_render::color::colorspace::SrgbColorSpace::<u8>::linear_to_nonlinear_srgb`__

Modify the `u8` value to instead be an `f32` (`|x| x as f32 / 255.`), use `bevy_color::color::gamma_function_inverse`, and back again.

__`bevy_render::color::colorspace::SrgbColorSpace::<u8>::nonlinear_to_linear_srgb`__

Modify the `u8` value to instead be an `f32` (`|x| x as f32 / 255.`), use `bevy_color::color::gamma_function`, and back again.

__`bevy_render::color::colorspace::HslRepresentation::hsl_to_nonlinear_srgb`__

Use `Hsla`’s implementation of `Into<Srgba>`

__`bevy_render::color::colorspace::HslRepresentation::nonlinear_srgb_to_hsl`__

Use `Srgba`’s implementation of `Into<Hsla>`

__`bevy_render::color::colorspace::LchRepresentation::lch_to_nonlinear_srgb`__

Use `Lcha`’s implementation of `Into<Srgba>`

__`bevy_render::color::colorspace::LchRepresentation::nonlinear_srgb_to_lch`__

Use `Srgba`’s implementation of `Into<Lcha>`

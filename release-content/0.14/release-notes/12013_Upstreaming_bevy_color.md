Colors are a huge part of building a good game: UI, effects, shaders and more all need fully-feature, correct and convenient color tools.
Bevy now supports a wide range of color spaces, each with their own type (e.g. `LinearRgba`, `Hsla` or `Oklaba`),
and offers a wide range of fully documented operations on and conversions between them.

Critically, color mixing / blending is now supported: perfect for procedurally generating color palettes and working with animations.

```rust
use bevy_color::*;

// Each color space now corresponds to a specific type
let red = Srgba::rgb(1., 0., 0.);

// All non-standard color space conversions are done through the `LinearRgba` color space
// to avoid a quadratic explosion of generated code.
let red = Oklcha::from(LinearRgba::from(red));

// We've added the `tailwind` palette colors: perfect for quick-but-pretty prototyping!
let blue = Oklcha = tailwind::BLUE_500;

// The color space that you're mixing your colors in has a huge impact!
// Consider using the scientifically-motivated `Oklcha` or `Oklaba` for a perceptually linear effect
let purple =  red.mix(blue, 0.5);
```

Most of the user-facing APIs still accept a colorspace-agnostic `Color` (which now wraps our color-space types),
while rendering internals use the physically-based `LinearRgba` type.

`bevy_color` offers a solid, type-safe foundation but it's just getting started.
If there are more things you'd like to do to your colors, please open an issue or PR and we'd be happy to help!

P.S. `bevy_color` is intended to operate effectively as a stand-alone crate: feel free to take a dependency on it for your non-Bevy projects as well.

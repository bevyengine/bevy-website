Colors are a huge part of building a good game: UI, effects, shaders and more all need fully-featured, correct and convenient color tools.

Bevy now supports a broad selection of color spaces, each with their own type (e.g. `LinearRgba`, `Hsla`, `Oklaba`),
and offers a wide range of fully documented operations on and conversions between them.

The new API is more error-resistant, more idiomatic and allows us to save work by storing the `LinearRgba` type in our rendering internals.
This solid foundation has allowed us to implement a wide range of useful operations, clustered into traits like `Hue` or `Alpha`,
allowing you to operate over any color space with the required property.
Critically, color mixing / blending is now supported: perfect for procedurally generating color palettes and working with animations.

```rust
use bevy_color::prelude::*;

// Each color space now corresponds to a specific type
let red = Srgba::rgb(1., 0., 0.);

// All non-standard color space conversions are done through the shortest path between
// the source and target color spaces to avoid a quadratic explosion of generated code.
// This conversion...
let red = Oklcha::from(red);
// ...is implemented using
let red = Oklcha::from(Oklaba::from(LinearRgba::from(red)));

// We've added the `tailwind` palette colors: perfect for quick-but-pretty prototyping!
// And the existing CSS palette is now actually consistent with the industry standard :p
let blue = tailwind::BLUE_500;

// The color space that you're mixing your colors in has a huge impact!
// Consider using the scientifically-motivated `Oklcha` or `Oklaba` for a perceptually uniform effect.
let purple = red.mix(blue, 0.5);
```

Most of the user-facing APIs still accept a colorspace-agnostic `Color` (which now wraps our color-space types),
while rendering internals use the physically-based `LinearRgba` type.
For an overview of the different color spaces, and what they're each good for, please check out our [color space usage](https://dev-docs.bevyengine.org/bevy/color/index.html#color-space-usage) documentation.

`bevy_color` offers a solid, type-safe foundation, but it's just getting started.
If you'd like another color space or there are more things you'd like to do to your colors, please open an issue or PR and we'd be happy to help!

P.S. `bevy_color` is intended to operate effectively as a stand-alone crate: feel free to take a dependency on it for your non-Bevy projects as well.

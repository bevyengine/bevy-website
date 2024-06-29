<!-- Implement filmic color grading. -->
<!-- https://github.com/bevyengine/bevy/pull/13121 -->

Artists want to get exactly the right look for their game, and color plays a huge role.

To support this, Bevy's [existing tonemapping tools](https://bevyengine.org/news/bevy-0-10/#more-tonemapping-choices) have been extended to include a complete set of filmic color grading tools. In addition to a [base tonemap](https://dev-docs.bevyengine.org/bevy/core_pipeline/tonemapping/enum.Tonemapping.html), you can now configure:

- White point adjustment. This is inspired by Unity's implementation of the feature, but simplified and optimized. Temperature and tint control the adjustments to the x and y chromaticity values of CIE 1931. Following Unity, the adjustments are made relative to the D65 standard illuminant in the LMS color space.
- Hue rotation: converts the RGB value to HSV, alters the hue, and converts back.
- Color correction: allows the gamma, gain, and lift values to be adjusted according to the standard ASC CDL combined function. This can be done separately for shadows, midtones and highlights To avoid abrupt color changes, a small crossfade is used between the different sections of the image.

We've followed [Blender's](https://www.blender.org/) implementation as closely as possible to ensure that what you see in your modelling software matches what you see in the game.

![A very orange image of a test scene, with controls for exposure, temperature, tint and hue. Saturation, contrast, gamma, gain, and lift can all be configured for the highlights, midtones, and shadows separately.](filmic_color_grading.png)

We've provided a new, [`color_grading`](https://github.com/bevyengine/bevy/blob/main/examples/3d/color_grading.rs) example, with a shiny GUI to change all the color grading settings.
Perfect for copy-pasting into your own game's dev tools and playing with the settings!
Note that these settings can all be changed at runtime: giving artists control over the exact mood of the scene, or shift it dynamically based on weather or time of day.

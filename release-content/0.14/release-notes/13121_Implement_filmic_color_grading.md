<!-- Implement filmic color grading. -->
<!-- https://github.com/bevyengine/bevy/pull/13121 -->

> This commit expands Bevy's existing tonemapping feature to a complete set of filmic color grading tools, matching those of engines like Unity, Unreal, and Godot. The following features are supported:
>
> White point adjustment. This is inspired by Unity's implementation of the feature, but simplified and optimized. Temperature and tint control the adjustments to the x and y chromaticity values of CIE 1931. Following Unity, the adjustments are made relative to the D65 standard illuminant in the LMS color space.
>
> Hue rotation. This simply converts the RGB value to HSV, alters the hue, and converts back.
>
> Color correction. This allows the gamma, gain, and lift values to be adjusted according to the standard ASC CDL combined function.
>
> Separate color correction for shadows, midtones, and highlights. Blender's source code was used as a reference for the implementation of this. The midtone ranges can be adjusted by the user. To avoid abrupt color changes, a small crossfade is used between the different sections of the image, again following Blender's formulas.
>
> A new example, color_grading, has been added, offering a GUI to change all the color grading settings. It uses the same test scene as the existing tonemapping example, which has been factored out into a shared glTF scene.
>
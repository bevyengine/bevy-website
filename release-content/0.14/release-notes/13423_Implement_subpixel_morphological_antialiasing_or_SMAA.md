<!-- Implement subpixel morphological antialiasing, or SMAA. -->
<!-- https://github.com/bevyengine/bevy/pull/13423 -->

<!-- TODO -->

> This commit implements a large subset of subpixel morphological antialiasing, better known as SMAA. SMAA is a 2011 antialiasing technique that detects jaggies in an aliased image and smooths them out. Despite its age, it's been a continual staple of games for over a decade. Four quality presets are available: low, medium, high, and ultra. I set the default to high, on account of modern GPUs being significantly faster than they were in 2011.
>
> Like the already-implemented FXAA, SMAA works on an unaliased image. Unlike FXAA, it requires three passes: (1) edge detection; (2) blending weight calculation; (3) neighborhood blending. Each of the first two passes writes an intermediate texture for use by the next pass. The first pass also writes to a stencil buffer in order to dramatically reduce the number of pixels that the second pass has to examine. Also unlike FXAA, two built-in lookup textures are required; I bundle them into the library in compressed KTX2 format.

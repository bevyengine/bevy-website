Not all fog is created equal.
Bevy's existing implementation covers [distance fog](https://en.wikipedia.org/wiki/Distance_fog), which is fast, simple, and not particularly realistic.

In Bevy 0.14, this is supplemented with volumetric fog, based on [volumetric lighting](https://en.wikipedia.org/wiki/Volumetric_lighting), which simulates fog using actual 3D space, rather than simply distance from the camera.
As you might expect, this is both prettier and more computationally expensive!

In particular, this allows for the creation of stunningly beautiful "god rays" (more properly, crepuscular rays) shining through the fog.

{{ compare_slider(
    path="POST_PATH",
    left_title="Without Volumetric Fog",
    left_image="without_volumetric_fog.png",
    right_title="With Volumetric Fog",
    right_image="with_volumetric_fog.png"
) }}

Bevy's algorithm, which is implemented as a postprocessing effect, is a combination of the techniques described in [Scratchapixel](https://www.scratchapixel.com/lessons/3d-basic-rendering/volume-rendering-for-developers/intro-volume-rendering.html) and [Alexandre Pestana's blog post](https://www.alexandre-pestana.com/volumetric-lights/). It uses raymarching ([ported to WGSL by h3r2tic](https://gist.github.com/h3r2tic/9c8356bdaefbe80b1a22ae0aaee192db)) in screen space, transformed into shadow map space for sampling and combined with physically-based modeling of absorption and scattering. Bevy employs the widely-used Henyey-Greenstein phase function to model asymmetry; this essentially allows light shafts to fade into and out of existence as the user views them.

To add volumetric fog to a scene, add `VolumetricFogSettings` to the camera, and add `VolumetricLight` to directional lights that you wish to be volumetric. `VolumetricFogSettings` has numerous settings that allow you to define the accuracy of the simulation, as well as the look of the fog. Currently, only interaction with directional lights that have shadow maps is supported. Note that the overhead of the effect scales directly with the number of directional lights in use, so apply `VolumetricLight` sparingly for the best results.

Try it hands on with our [`volumetric_fog` example](https://github.com/bevyengine/bevy/tree/v0.14.0/examples/3d/volumetric_fog.rs).

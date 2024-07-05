<!-- Implement subpixel morphological antialiasing, or SMAA. -->
<!-- https://github.com/bevyengine/bevy/pull/13423 -->

Jagged edges are the bane of game developers' existence: a wide variety of anti-aliasing techniques have been invented and are still in use to fix them without degrading image quality.
In addition to [MSAA](https://en.wikipedia.org/wiki/Multisample_anti-aliasing), [FXAA](https://en.wikipedia.org/wiki/Fast_approximate_anti-aliasing), and [TAA](https://en.wikipedia.org/wiki/Temporal_anti-aliasing), Bevy now implements [SMAA](https://en.wikipedia.org/wiki/Morphological_antialiasing): subpixel morphological antialiasing.

SMAA is a 2011 antialiasing technique that detects borders in the image, then averages nearby border pixels, eliminating the dreaded jaggies.
Despite its age, it's been a continual staple of games for over a decade. Four quality presets are available: low, medium, high, and ultra. Due to advancements in consumer hardware, Bevy's default is high.

You can see how it compares to no anti-aliasing in the pair of images below:

{{ compare_slider(
    path="POST_PATH",
    left_title="No AA",
    left_image="no_aa.jpg",
    right_title="SMAA",
    right_image="smaa.jpg"
) }}

The best way to get a sense for the tradeoffs of the various anti-aliasing methods is to experiment with a test scene using the [`anti_aliasing` example](https://github.com/bevyengine/bevy/tree/v0.14.0/examples/3d/anti_aliasing.rs) or by simply trying it out in your own game.

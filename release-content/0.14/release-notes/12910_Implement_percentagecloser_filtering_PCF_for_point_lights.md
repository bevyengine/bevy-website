Percentage-closer filtering is a standard anti-aliasing technique used to get softer, less jagged shadows.
To do so, we sample from the shadow map near the pixel of interest using a Gaussian kernel, averaging the results to reduce sudden transitions as we move in / out of the shadow.

As a result, Bevy's point lights now  look softer and more natural, without any changes to end user code. As before, you can configure the exact strategy used to alias your shadows by setting the [`ShadowFilteringMethod`](https://dev-docs.bevyengine.org/bevy/pbr/enum.ShadowFilteringMethod.html) component on your 3D cameras.

TODO: add image from https://github.com/bevyengine/bevy/pull/12910

Full support for percentage-closer shadows is [in the works](https://github.com/bevyengine/bevy/pull/13497): testing and reviews for this are, like always, extremely welcome.

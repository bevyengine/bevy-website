Percentage-closer filtering is a standard anti-aliasing technique used to get softer, less jagged shadows.
To do so, we sample from the shadow map near the pixel of interest using a Gaussian kernel, averaging the results to reduce sudden transitions as we move in / out of the shadow.

As a result, Bevy's point lights now  look softer and more natural, without any changes to end user code. As before, you can configure the exact strategy used to anti-alias your shadows by setting the [`ShadowFilteringMethod`](https://docs.rs/bevy/0.14/bevy/pbr/enum.ShadowFilteringMethod.html) component on your 3D cameras.

{{ compare_slider(
    path="POST_PATH",
    left_title="Without PCF filtering",
    left_image="before_pcf.png",
    right_title="With PCF filtering",
    right_image="after_pcf.png"
) }}

Full support for percentage-closer shadows is [in the works](https://github.com/bevyengine/bevy/pull/13497): testing and reviews for this are, like always, extremely welcome.

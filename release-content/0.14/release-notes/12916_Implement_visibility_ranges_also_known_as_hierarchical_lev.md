<!-- Implement visibility ranges, also known as hierarchical levels of detail (HLODs). -->
<!-- https://github.com/bevyengine/bevy/pull/12916 -->

When looking at objects far away, it's hard to make out the details!
This obvious fact is just as true in rendering as it is in real life.
As a result, using complex, high-fidelity models for distant objects is a waste: we can replace their meshes with simplified equivalents.

By automatically varying the **level-of-detail** (LOD) of our models in this way, we can render much larger scenes (or the same open world with a higher draw distance), swapping out meshes on the fly based on their proximity to the player.
Bevy now supports one of the most foundational tools for this: **visibility ranges** (sometimes called hierarchical levels of detail, as it allows users to replace multiple meshes with a single object).

By setting the [`VisibilityRange`] component on your mesh entities, developers can automatically control the range from the camera at which their meshes will appear and disappear, automatically fading between the two options using dithering.
Hiding meshes happens early in the rendering pipeline, so this feature can be efficiently used for level of detail optimization.
As a bonus, this feature is properly evaluated per-view, so different views can show different levels of detail.

Note that this feature differs from proper mesh LODs (where the geometry itself is simplified automatically), which will come later.
While mesh LODs are useful for optimization and don't require any additional setup, they're less flexible than visibility ranges.
Games often want to use objects other than meshes to replace distant models, such as octahedral or [billboard](https://github.com/bevyengine/bevy/issues/3688) imposters: implementing visibility ranges first gives users the flexibility to start implementing these solutions today.

You can see how this feature is used in the [`visibility_range` example](https://github.com/bevyengine/bevy/tree/v0.14.0/examples/3d/visibility_range.rs).

[`VisibilityRange`]: https://docs.rs/bevy/0.14/bevy/render/view/struct.VisibilityRange.html
